use crate::data::{User, UserCreateRequest, UserUpdateRequest};
use crate::error::Error::{DBPoolError, DBQueryError};
use crate::{DBCon, DBPool, Result};
use chrono::{DateTime, Utc};
use mobc_postgres::tokio_postgres::Row;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls};

pub mod migration;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

const TABLE: &str = "users";

pub fn create_pool(conn_string: &String) -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str(conn_string)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(DBPool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn get_db_con(db_pool: &DBPool) -> std::result::Result<DBCon, crate::error::Error> {
    db_pool.get().await.map_err(DBPoolError)
}

pub async fn check_db(db_pool: &DBPool) -> std::result::Result<(), crate::error::Error> {
    let con = get_db_con(&db_pool).await?;
    con.execute("SELECT 1", &[]).await.map_err(DBQueryError)?;
    Ok(())
}

fn row_to_user(row: &Row) -> User {
    let id: i32 = row.get(0);
    let username: String = row.get(1);
    let firstname: String = row.get(2);
    let lastname: String = row.get(3);
    let email: String = row.get(4);
    let phone: String = row.get(5);
    let created_at: DateTime<Utc> = row.get(6);
    let updated_at: DateTime<Utc> = row.get(7);

    User {
        id,
        username,
        firstname,
        lastname,
        email,
        phone,
        created_at,
        updated_at,
    }
}

pub async fn create_user(db_pool: &DBPool, body: UserCreateRequest) -> Result<User> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (username, firstname, lastname, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING *", TABLE);
    let row = con
        .query_one(
            query.as_str(),
            &[
                &body.username,
                &body.firstname,
                &body.lastname,
                &body.email,
                &body.phone,
            ],
        )
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_user(&row))
}

pub async fn get_users(db_pool: &DBPool) -> Result<Vec<User>> {
    println!("GET /user");
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT * FROM {} ORDER BY id ASC", TABLE);
    let rows = con.query(query.as_str(), &[]).await.map_err(DBQueryError)?;
    println!("Fetched rows: {:?}", rows);
    Ok(rows.iter().map(|r| row_to_user(&r)).collect())
}

pub async fn get_user(db_pool: &DBPool, id: i32) -> Result<Option<User>> {
    println!("GET /user/{:?}", id);
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
    let row = con
        .query_opt(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;

    println!("Fetched row: {:?}", row);
    Ok(row.map(|r| row_to_user(&r)))
}

pub async fn update_user(db_pool: &DBPool, id: i32, body: UserUpdateRequest) -> Result<User> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET username = $1, firstname = $2, lastname = $3, email = $4, phone = $5, updated_at = $6 WHERE id = $7 RETURNING *",
        TABLE
    );
    let now = Utc::now();
    let row = con
        .query_one(
            query.as_str(),
            &[
                &body.username,
                &body.firstname,
                &body.lastname,
                &body.email,
                &body.phone,
                &now,
                &id,
            ],
        )
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_user(&row))
}

pub async fn delete_user(db_pool: &DBPool, id: i32) -> Result<bool> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    let res = con
        .execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;
    Ok(res > 0)
}
