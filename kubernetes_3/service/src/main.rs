use service::config::Config;
use service::run;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    println!("Starting server on port {}", config.port);
    run(&config).await
}
