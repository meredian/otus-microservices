use crate::{db, DBPool, Result};
use std::convert::Infallible;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use warp::{http::StatusCode, reject, Reply, Rejection};

use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use warp::reply::json;
use crate::data::{TodoRequest, TodoResponse, TodoUpdateRequest};

#[derive(Debug, Serialize)]
struct HealthCheckReply {
    status: String,
    db: String,
}

pub async fn root_handler() -> Result<impl Reply> {
    Ok("Hello, world!")
}

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
    // let res = db::get_db_con(&db_pool).await;
    // let res = then(db_future, |db| db.execute("SELECT 1", &[]))

    // let res: () = db.execute("SELECT 1", &[]).await;

    let response = HealthCheckReply {
        status: String::from("OK"),
        db: String::from("OK"),
        // db: match res {

        // }
    };
    Ok(json(&response))
}

pub async fn create_todo_handler(body: TodoRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        db::create_todo(&db_pool, body).await?
    )))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn list_todos_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let todos = db::fetch_todos(&db_pool, query.search).await?;
    Ok(json::<Vec<_>>(
        &todos.into_iter().map(|t| TodoResponse::of(t)).collect(),
    ))
}

pub async fn update_todo_handler(
    id: i32,
    body: TodoUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        db::update_todo(&db_pool, id, body).await?,
    )))
}

pub async fn delete_todo_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::delete_todo(&db_pool, id).await?;
    Ok(StatusCode::OK)
}