use service::config::Config;
use service::{migrate, wait_for_migrate};
use std::env;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Running migrations...");
        migrate(&config).await;
        println!("Running migrations...DONE");
    } else if args.len() == 2 && args[1].eq("--wait") {
        println!("Waiting for migration to be applied...");
        wait_for_migrate(&config).await;
        println!("Waiting for migration to be applied...DONE");
    } else {
        panic!("Unexpected arguments, only optional \"--wait\" can be passed");
    }
}
