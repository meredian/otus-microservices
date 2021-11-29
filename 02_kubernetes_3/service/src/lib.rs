use std::convert::Infallible;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, Rejection>;

use mobc::{Connection, Pool};
use mobc_postgres::{
    tokio_postgres::{self},
    PgConnectionManager,
};
use tokio_postgres::NoTls;

type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

pub mod config;
mod data;
mod db;
mod error;
mod handler;

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub async fn migrate(config: &config::Config) {
    let db_pool = db::create_pool(&config.db_conn_string).expect("database pool can be created");

    db::migration::migrate(&db_pool)
        .await
        .expect("failed to migrate database");
}

pub async fn wait_for_migrate(config: &config::Config) {
    let db_pool = db::create_pool(&config.db_conn_string).expect("database pool can be created");

    db::migration::wait_for_migrate(&db_pool)
        .await
        .expect("failed to wait for database migration");
}

pub async fn run(config: &config::Config) {
    let db_pool = db::create_pool(&config.db_conn_string).expect("database pool can be created");

    pretty_env_logger::init();
    let log = warp::log("api");

    let routes = handler::router(&db_pool)
        .with(log)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    println!("Starting server on port {}", config.port);
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
}
