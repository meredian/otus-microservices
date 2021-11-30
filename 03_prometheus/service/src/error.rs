use mobc_postgres::tokio_postgres;
use thiserror::Error;

use serde::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, reply, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("error creating DB pool: {0}")]
    DBCreatePoolError(tokio_postgres::Error),
    #[error("error getting connection from DB pool: {0}")]
    DBPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("error initialising database: {0}")]
    DBInitError(tokio_postgres::Error),
    #[error("error running {0} migration on database: {1}")]
    DBMigrateError(String, tokio_postgres::Error),
    #[error("migration for record from database \"{0}\" not found in migration list")]
    DBMigrationNotFoundError(String),
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
    #[error("error reading path from directory: {0}")]
    DirectoryListError(std::io::Error),
    // We introduce custom NotFound type since
    #[error("User with id {0} not found")]
    UserNotFound(i32),
}

impl Reply for Error {
    fn into_response(self) -> reply::Response {
        let (code, message) = map_error(&self);
        error_reply(code, message).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

fn map_error(err: &Error) -> (StatusCode, &'static str) {
    match err {
        Error::UserNotFound(_) => (StatusCode::NOT_FOUND, "User not found"),
        Error::DBQueryError(_) => (StatusCode::BAD_REQUEST, "Could not Execute request"),
        _ => {
            eprintln!("unhandled application error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        }
    }
}

fn error_reply(code: StatusCode, message: &str) -> impl Reply {
    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    return warp::reply::with_status(json, code);
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    println!("Error: {:?}", err);
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
        let (mapped_code, mapped_message) = map_error(e);
        code = mapped_code;
        message = mapped_message;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    Ok(error_reply(code, message))
}
