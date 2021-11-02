use crate::{DBCon, DBPool, Result};
use crate::error::Error::{DBInitError, DBPoolError, DBQueryError};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use chrono::{DateTime, Utc};
use mobc_postgres::tokio_postgres::Row;
use warp::Rejection;
use crate::data::{Todo, TodoRequest, TodoUpdateRequest};

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

pub async fn get_db_con(db_pool: &DBPool) -> std::result::Result<DBCon, crate::error::Error> {
    db_pool.get().await.map_err(DBPoolError)
}

pub async fn init_db(db_pool: &DBPool) -> std::result::Result<(), crate::error::Error> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    println!("Init file: {}", init_file);
    let con = get_db_con(db_pool).await?;
    con
            .batch_execute(init_file.as_str())
            .await
            .map_err(DBInitError)?;
    Ok(())
}

const INIT_SQL: &str = "./migrations/init.sql";
const TABLE: &str = "todo";

pub async fn create_todo(db_pool: &DBPool, body: TodoRequest) -> Result<Todo> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let checked: bool = row.get(3);
    Todo {
        id,
        name,
        created_at,
        checked,
    }
}

const SELECT_FIELDS: &str = "id, name, created_at, checked";

pub async fn fetch_todos(db_pool: &DBPool, search: Option<String>) -> Result<Vec<Todo>> {
    let con = get_db_con(db_pool).await?;
    let where_clause = match search {
        Some(_) => "WHERE name like $1",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {} ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE, where_clause
    );
    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
}

pub async fn update_todo(db_pool: &DBPool, id: i32, body: TodoUpdateRequest) -> Result<Todo> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET name = $1, checked = $2 WHERE id = $3 RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.checked, &id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn delete_todo(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(|e| warp::reject::custom(DBQueryError(e)))
}
