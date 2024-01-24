mod config;
mod command;
mod server;
mod logger;

use crate::config::Config;
use crate::server::run_server;
use std::fs;

#[tokio::main]
async fn main() {
    logger::init(); // Initialize the logger
    let config_contents = fs::read_to_string("config.yaml").expect("Failed to read config file");
    let config: Config = serde_yaml::from_str(&config_contents).expect("Failed to parse config file");

    run_server(config).await;
}