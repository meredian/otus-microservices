use crate::{db, DBPool, Result};
use std::convert::Infallible;
use serde::{Serialize};
use crate::error::Error;
use warp::{http::StatusCode, reject, Reply, Rejection};

use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct HealthCheckReply {
    status: String,
    db: String,
}

pub async fn root_handler() -> Result<impl Reply> {
    Ok("Hello, world!")
}

// async fn run_db_query<T>(db_pool: DBPool, query: String, params: T[]) -> Result<T> {
//     let db = db::get_db_con(&db_pool).await?;

//     let res: () = db.execute("SELECT 1", &[]).await;
//     Ot
// }

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
    Ok(warp::reply::json(&response))

}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
            code = StatusCode::NOT_FOUND;
            message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
            code = StatusCode::BAD_REQUEST;
            message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
            match e {
                Error::DBQueryError(_) => {
                    code = StatusCode::BAD_REQUEST;
                    message = "Could not Execute request";
                }
                _ => {
                    eprintln!("unhandled application error: {:?}", err);
                    code = StatusCode::INTERNAL_SERVER_ERROR;
                    message = "Internal Server Error";
                }
            }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
            code = StatusCode::METHOD_NOT_ALLOWED;
            message = "Method Not Allowed";
    } else {
            eprintln!("unhandled error: {:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
            message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
