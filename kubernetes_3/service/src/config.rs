use std::env;

const DEFAULT_PORT: u16 = 3000;

pub struct Config {
    pub port: u16,
    pub db_conn_string: String,
}

impl Config {
    pub fn from_env() -> Config {
        let port = match env::var("PORT") {
            Ok(s) => s.parse::<u16>().unwrap(),
            Err(_) => DEFAULT_PORT,
        };
        let db_conn_string = String::from("postgres://postgres:pwd@127.0.0.1:7878/postgres");

        Config {
            port,
            db_conn_string,
        }
    }
}
