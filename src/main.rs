mod config;
mod command;
mod server;
mod logger;

use crate::config::Config;
use crate::server::run_server;
use std::fs;
use command::{CommandRegistry, Command, HelloMessage};
use std::sync::Arc;

#[macro_export]
macro_rules! register_commands {
    ($registry:expr, $( $cmd_id:expr => $cmd_type:ty ),*) => {
        $(
            $registry.register($cmd_id, |data| Box::new(serde_json::from_str::<$cmd_type>(data).unwrap()) as Box<dyn Command>);
        )*
    };
}


#[tokio::main]
async fn main() {
    logger::init(); // Initialize the logger
    let config_contents = fs::read_to_string("config.yaml").expect("Failed to read config file");
    let config: Config = serde_yaml::from_str(&config_contents).expect("Failed to parse config file");

    let mut registry = CommandRegistry::new();
    register_commands!(registry,
        1 => HelloMessage
        // Add more commands here
    );

    let arc_registry = Arc::new(registry);
    run_server(config, arc_registry).await;
}