use crate::{DBCon, DBPool};
use crate::error::Error::{DBInitError, DBPoolError};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls};
use std::fs;
use std::str::FromStr;
use std::time::Duration;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub fn create_pool(conn_string: &String) -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str(conn_string)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(DBPool::builder()
            .max_open(DB_POOL_MAX_OPEN)
            .max_idle(DB_POOL_MAX_IDLE)
            .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
            .build(manager))
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon, crate::error::Error> {
    db_pool.get().await.map_err(DBPoolError)
}

pub async fn init_db(db_pool: &DBPool) -> Result<(), crate::error::Error> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con
            .batch_execute(init_file.as_str())
            .await
            .map_err(DBInitError)?;
    Ok(())
}

const INIT_SQL: &str = "./migrations/init.sql";
