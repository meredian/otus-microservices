use std::fs;
use std::vec;

use super::get_db_con;
use crate::{
    error::Error,
    error::Error::{DBInitError, DBMigrateError, DBMigrationNotFoundError, DBQueryError},
    DBCon, DBPool,
};
use chrono::prelude::*;
use mobc_postgres::tokio_postgres::Row;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

const WAIT_LOOP_DURATION: Duration = Duration::from_secs(1);

#[derive(Deserialize, Debug)]
struct MigrationRecord {
    pub migration_id: String,
    pub migrated_at: DateTime<Utc>,
}

#[derive(Debug)]
struct Migration {
    pub migration_id: String,
    pub migration_sql: String,
}

const INIT_MIGRATION: &str = "
    CREATE TABLE IF NOT EXISTS migrations (
        migration_id TEXT PRIMARY KEY NOT NULL,
        migrated_at timestamp with time zone DEFAULT (now() at time zone 'utc')
    );
";

pub async fn migrate(db_pool: &DBPool) -> Result<(), Error> {
    let db_con = get_db_con(db_pool).await?;
    init_migration_table(&db_con).await?;
    apply_all_migrations(&db_con).await?;
    Ok(())
}

pub async fn wait_for_migrate(db_pool: &DBPool) -> Result<(), Error> {
    let db_con = get_db_con(db_pool).await?;
    let last_migration_id = get_last_migration_from_disk()
        .await?
        .map(|m| m.migration_id);
    let last_migration_record_id = get_last_applied_migration_record(&db_con)
        .await?
        .map(|r| r.migration_id);

    println!("Waiting for migration {:?}", last_migration_id);
    match (last_migration_id, last_migration_record_id) {
        (None, None) => Ok(()),
        (None, Some(r_id)) => Err(DBMigrationNotFoundError(r_id)),
        (Some(m_id), None) => loop_wait_for_migration_applied(&db_con, &m_id).await,
        (Some(m_id), Some(r_id)) => {
            if m_id.eq(&r_id) {
                Ok(())
            } else if m_id.lt(&r_id) {
                Err(DBMigrationNotFoundError(r_id))
            } else {
                loop_wait_for_migration_applied(&db_con, &m_id).await
            }
        }
    }
}

async fn loop_wait_for_migration_applied(
    db_con: &DBCon,
    migration_id: &String,
) -> Result<(), Error> {
    loop {
        let last_migration_record_id = get_last_applied_migration_record(&db_con)
            .await?
            .map(|r| r.migration_id);

        if let Some(r_id) = last_migration_record_id {
            if migration_id.eq(&r_id) {
                return Ok(());
            } else if migration_id.lt(&r_id) {
                return Err(DBMigrationNotFoundError(r_id));
            }
        }
        sleep(WAIT_LOOP_DURATION).await;
    }
}

async fn init_migration_table(db_con: &DBCon) -> Result<(), Error> {
    match db_exec(db_con, INIT_MIGRATION).await {
        Err(DBQueryError(err)) => Err(DBInitError(err)),
        Err(err) => Err(err),
        Ok(()) => Ok(()),
    }
}

async fn apply_all_migrations(db_con: &DBCon) -> Result<(), Error> {
    let migrations = read_migrations_from_disk().await?;
    db_exec(
        db_con,
        "BEGIN; LOCK TABLE migrations IN ACCESS EXCLUSIVE MODE",
    )
    .await?;

    let migrations = get_unapplied_migrations(db_con, migrations).await?;
    println!("Migrations to apply: {:?}", migrations);
    if migrations.len() == 0 {
        return db_exec(db_con, "COMMIT").await;
    }

    if let Err(err) = apply_migrations(db_con, migrations).await {
        if let DBMigrateError(_, _) = err {
            db_exec(db_con, "ROLLBACK").await?;
        };
        return Err(err);
    } else {
        db_exec(db_con, "COMMIT").await?;
    }

    Ok(())
}

async fn db_exec(db_con: &DBCon, sql: &str) -> Result<(), Error> {
    println!("Executing SQL: {}", sql);
    db_con.batch_execute(sql).await.map_err(DBQueryError)?;
    Ok(())
}

async fn get_unapplied_migrations(
    db_con: &DBCon,
    migrations: Vec<Migration>,
) -> Result<Vec<Migration>, Error> {
    let last_record = get_last_applied_migration_record(db_con).await?;
    println!("Last record: {:?}", last_record);
    match last_record {
        None => Ok(migrations),
        Some(rec) => {
            let position = migrations
                .iter()
                .position(|m| m.migration_id.eq(&rec.migration_id));
            match position {
                None => Err(DBMigrationNotFoundError(rec.migration_id)),
                Some(pos) => Ok(migrations.into_iter().skip(pos + 1).collect()),
            }
        }
    }
}

async fn get_last_applied_migration_record(
    db_con: &DBCon,
) -> Result<Option<MigrationRecord>, Error> {
    let query = "SELECT * FROM migrations ORDER BY migration_id DESC LIMIT 1";
    let row = db_con.query_opt(query, &[]).await.map_err(DBQueryError)?;
    Ok(row.map(|r| row_to_migration_record(&r)))
}

fn row_to_migration_record(row: &Row) -> MigrationRecord {
    let migration_id = row.get(0);
    let migrated_at = row.get(1);
    MigrationRecord {
        migration_id,
        migrated_at,
    }
}

async fn apply_migrations(db_con: &DBCon, migrations: Vec<Migration>) -> Result<(), Error> {
    for migration in migrations {
        apply_migration(db_con, migration).await?
    }
    Ok(())
}

async fn apply_migration(db_con: &DBCon, migration: Migration) -> Result<(), Error> {
    let sql = format!(
        "INSERT INTO migrations (migration_id) VALUES ('{}');",
        migration.migration_id
    );
    match db_exec(db_con, &sql).await {
        Err(DBQueryError(err)) => Err(DBMigrateError(migration.migration_id, err)),
        Err(err) => Err(err),
        Ok(()) => match db_exec(db_con, &migration.migration_sql).await {
            Err(DBQueryError(err)) => Err(DBMigrateError(migration.migration_id, err)),
            Err(err) => Err(err),
            Ok(()) => Ok(()),
        },
    }
}

async fn read_migrations_from_disk() -> Result<Vec<Migration>, Error> {
    let paths = fs::read_dir("./migrations")?;

    let mut migrations: Vec<Migration> = vec![];
    for path in paths {
        let filepath = path.map_err(Error::DirectoryListError)?.path();
        if let Some(file_name) = filepath.file_stem() {
            let migration_sql = fs::read_to_string(&filepath).map_err(Error::ReadFileError)?;
            migrations.push(Migration {
                migration_id: file_name.to_string_lossy().to_string(),
                migration_sql,
            })
        }
    }

    migrations.sort_by(|a, b| {
        a.migration_id
            .to_lowercase()
            .cmp(&b.migration_id.to_lowercase())
    });

    Ok(migrations)
}

async fn get_last_migration_from_disk() -> Result<Option<Migration>, Error> {
    Ok(read_migrations_from_disk().await?.pop())
}
