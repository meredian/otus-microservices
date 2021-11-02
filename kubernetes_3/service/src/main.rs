use std::env;
use std::convert::Infallible;
use warp::http::status::StatusCode;
use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;

type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

mod data;
mod db;
mod error;
mod handler;

const DEFAULT_PORT: u16 = 3000;

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(s) => s.parse::<u16>().unwrap(),
        Err(_) => DEFAULT_PORT,
    };
    let conn_string = String::from("postgres://postgres@127.0.0.1:7878/postgres");
    let db_pool = db::create_pool(&conn_string)
        .expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    pretty_env_logger::init();
    let log = warp::log("api");

    let root_route = warp::path::end().and(warp::get()).and_then(handler::root_handler);
    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handler::health_handler);

    let routes = root_route
        .or(health_route)
        .with(log)
        .with(warp::cors()
        .allow_any_origin())
        .recover(recover_handler);

    println!("Starting server on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

async fn recover_handler(_err: warp::Rejection) -> Result<impl warp::Reply> {
    Ok(StatusCode::NOT_FOUND)
}
