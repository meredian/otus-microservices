use std::env;

pub struct Config {
    pub port: u16,
    pub db_conn_string: String,
    pub env: String,
}

impl Config {
    pub fn from_env() -> Config {
        let port = env::var("PORT")
            .expect("PORT is not set")
            .parse::<u16>()
            .unwrap();
        let db_conn_string = env::var("PG_CONN_STRING").expect("PG_CONN_STRING is not set");
        let env = match env::var("RUST_ENV") {
            Ok(x) => x,
            Err(_) => "develop".to_owned(),
        };

        Config {
            port,
            db_conn_string,
            env,
        }
    }
}
