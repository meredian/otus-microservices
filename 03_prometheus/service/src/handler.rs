use std::convert::Infallible;

use crate::{db, with_db, DBPool, Result};
use serde::Serialize;
use warp::Filter;
use warp::{http::StatusCode, Reply};

use crate::data::{UserCreateRequest, UserCreateResponce, UserUpdateRequest, UserUpdateResponse};
use crate::error::Error;
use crate::metrics::REGISTRY;
use warp::reply::{json, with_status};

type InfalliableResult<T> = std::result::Result<T, Infallible>;

#[derive(Debug, Serialize)]
struct HealthCheckReply {
    status: String,
    db: String,
}

fn result_reply<T: Reply, E: Reply>(res: std::result::Result<T, E>) -> impl Reply {
    match res {
        Ok(r) => r.into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn root_handler() -> InfalliableResult<impl Reply> {
    Ok("Hello, world!")
}

pub async fn health_handler(db_pool: DBPool) -> InfalliableResult<impl Reply> {
    let mut is_ok = true;
    let db = match db::check_db(&db_pool).await {
        Ok(()) => "OK".into(),
        Err(e) => {
            is_ok = false;
            format!("FAIL: {}", e.to_string())
        }
    };
    let status = if is_ok { "OK".into() } else { "FAIL".into() };
    let status_code = if is_ok {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };
    Ok(with_status(
        json(&HealthCheckReply { status, db }),
        status_code,
    ))
}

async fn metrics_handler() -> InfalliableResult<impl Reply> {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };
    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    };
    let res_custom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_custom);
    Ok(res)
}

pub fn router(
    db_pool: &DBPool,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root_router = warp::path::end().and(warp::get()).and_then(root_handler);
    let health_router = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(health_handler);

    let metrics_router = warp::path!("metrics")
        .and(warp::get())
        .and_then(metrics_handler);

    root_router
        .or(health_router)
        .or(metrics_router)
        .or(user_router(&db_pool))
}

fn user_router(
    db_pool: &DBPool,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_users_route = warp::path!("user")
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .then(get_users_handler)
        .map(result_reply);

    let get_user_route = warp::path!("user" / i32)
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .then(get_user_handler)
        .map(result_reply);

    let create_user_route = warp::path!("user")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .then(create_user_handler)
        .map(result_reply);

    let update_user_route = warp::path!("user" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .then(update_user_handler)
        .map(result_reply);

    let delete_user_route = warp::path!("user" / i32)
        .and(warp::delete())
        .and(with_db(db_pool.clone()))
        .then(delete_user_handler)
        .map(result_reply);

    get_user_route
        .or(get_users_route)
        .or(create_user_route)
        .or(update_user_route)
        .or(delete_user_route)
}

pub async fn create_user_handler(body: UserCreateRequest, db_pool: DBPool) -> Result<impl Reply> {
    let user = db::create_user(&db_pool, body).await?;
    Ok(json(&UserCreateResponce::of(user)))
}

pub async fn get_users_handler(db_pool: DBPool) -> Result<impl Reply> {
    let todos = db::get_users(&db_pool).await?;
    Ok(json::<Vec<_>>(
        &todos
            .into_iter()
            .map(|t| UserUpdateResponse::of(t))
            .collect(),
    ))
}

pub async fn get_user_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let user = db::get_user(&db_pool, id).await?;
    match user {
        Some(user) => Ok(json(&UserUpdateResponse::of(user))),
        None => Err(Error::UserNotFound(id).into()),
    }
}

pub async fn update_user_handler(
    id: i32,
    body: UserUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    match db::update_user(&db_pool, id, body).await? {
        Some(u) => Ok(json(&UserUpdateResponse::of(u))),
        None => Err(Error::UserNotFound(id).into()),
    }
}

pub async fn delete_user_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    if db::delete_user(&db_pool, id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(Error::UserNotFound(id).into())
    }
}
