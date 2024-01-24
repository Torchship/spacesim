mod config;
mod command;
mod server;
mod logger;
mod simulation;
mod provider;

use crate::config::Config;
use crate::server::start_server;
use std::fs;
use command::{CommandRegistry, Command, HelloMessage};
use std::sync::Arc;
use simulation::{SpaceSimulation, PhysicsSimulation};
use tokio::task::JoinHandle;

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
        
    let server_handle = run_server(config);
    let simulation_handle = run_simulation();

    server_handle.await.unwrap();
    simulation_handle.await.unwrap();
}

fn run_server(config: Config) -> JoinHandle<()> {
    let mut command_registry = CommandRegistry::new();
    register_commands!(command_registry,
        1 => HelloMessage
        // Add more commands here
    );
    let command_registry = Arc::new(command_registry);

    return tokio::spawn(async {
        start_server(config, command_registry).await;
    });
}

fn run_simulation() -> JoinHandle<()> {
    return tokio::spawn(async {
        let mut simulation = SpaceSimulation::new();

        // let body_provider = Provider::<Body>::new();
        // body_provider.load();
        // body_provider.add_body(Body {
        //     position: Vec2 { x: 0.0, y: 0.0 },
        //     velocity: Vec2 { x: 1.0, y: 1.0 },
        //     mass: 5.0,
        // });

        simulation.start_simulation(30);
    });
}