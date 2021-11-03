use crate::{db, with_db, DBPool, Result};
use serde::Serialize;
use warp::Filter;
use warp::{http::StatusCode, Reply};

use crate::data::{UserCreateRequest, UserCreateResponce, UserUpdateRequest, UserUpdateResponse};
use crate::error::Error;
use warp::reply::{json, with_status};

#[derive(Debug, Serialize)]
struct HealthCheckReply {
    status: String,
    db: String,
}

pub async fn root_handler() -> Result<impl Reply> {
    Ok("Hello, world!")
}

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
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

pub fn router(
    db_pool: &DBPool,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root_router = warp::path::end().and(warp::get()).and_then(root_handler);
    let health_router = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(health_handler);

    root_router.or(health_router).or(user_router(&db_pool))
}

fn user_router(
    db_pool: &DBPool,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_users_route = warp::path!("user")
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(get_users_handler);

    let get_user_route = warp::path!("user" / i32)
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(get_user_handler);

    let create_user_route = warp::path!("user")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(create_user_handler);

    let update_user_route = warp::path!("user" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(update_user_handler);

    let delete_user_route = warp::path!("user" / i32)
        .and(warp::delete())
        .and(with_db(db_pool.clone()))
        .and_then(delete_user_handler);

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
        None => Err(Error::NotFound().into()),
    }
}

pub async fn update_user_handler(
    id: i32,
    body: UserUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&UserUpdateResponse::of(
        db::update_user(&db_pool, id, body).await?,
    )))
}

pub async fn delete_user_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::delete_user(&db_pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
