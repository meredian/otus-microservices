use std::convert::Infallible;
use warp::Filter;

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
mod metrics;

type Result<T> = std::result::Result<T, error::Error>;

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
    metrics::register_custom_metrics();
    let env = config.env.clone();
    let log = warp::log::custom(move |info| {
        let path = info.path();
        let method = info.method().as_str();
        metrics::track_request_time(info.elapsed().as_secs_f64(), method, path, &env);
        metrics::track_status_code(info.status().as_u16().into(), method, path, &env);

        println!(
            "{{\"method\":\"{}\",\"path\":\"{}\",\"status\":\"{}\",\"remote_addr\":\"{:?}\",\"version\":\"{:?}\",\"referer\":\"{:?}\",\"user_agent\":\"{:?}\",\"elapsed\":\"{:?}\",\"host\":\"{:?}\",\"request_headers\":{:?}}}",
            info.method(),
            info.path(),
            info.status(),
            info.remote_addr(),
            info.version(),
            info.referer(),
            info.user_agent(),
            info.elapsed(),
            info.host(),
            info.request_headers()
        );
    });
    let db_pool = db::create_pool(&config.db_conn_string).expect("database pool can be created");

    let routes = handler::router(&db_pool)
        .with(log)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    println!("Starting server on port {}", config.port);
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
}
