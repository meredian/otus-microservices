use std::convert::Infallible;
use std::env;
use warp::{Filter, Rejection};

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
    let conn_string = String::from("postgres://postgres:pwd@127.0.0.1:7878/postgres");
    let db_pool = db::create_pool(&conn_string).expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    pretty_env_logger::init();
    let log = warp::log("api");

    let routes = handler::router(&db_pool)
        .with(log)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    println!("Starting server on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
