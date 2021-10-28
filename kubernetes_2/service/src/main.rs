use std::env;
use serde::{Deserialize, Serialize};
use warp::http::status::StatusCode;
use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

const DEFAULT_PORT: u16 = 3000;

#[derive(Debug, Serialize, Deserialize)]
struct HealthCheckReply {
    status: String,
}

#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(s) => s.parse::<u16>().unwrap(),
        Err(_) => DEFAULT_PORT,
    };

    pretty_env_logger::init();
    let log = warp::log("api");

    let root_route = warp::path::end().and(warp::get()).and_then(root_handler);
    let health_route = warp::path!("health").and_then(health_handler);
    let routes = root_route
        .or(health_route)
        .with(log)
        .with(warp::cors()
        .allow_any_origin())
        .recover(recover_handler);


    println!("Starting server on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

async fn root_handler() -> Result<impl Reply> {
    Ok("Hello, world!")
}

async fn health_handler() -> Result<impl Reply> {
    let response = HealthCheckReply {
        status: String::from("OK"),
    };
    Ok(warp::reply::json(&response))
}

async fn recover_handler(_err: warp::Rejection) -> Result<impl warp::Reply> {
    Ok(StatusCode::NOT_FOUND)
}
