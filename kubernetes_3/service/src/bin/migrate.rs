use service::config::Config;
use service::migrate;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    println!("Running migrations");
    migrate(&config).await
}
