use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16
}

// Additional code related to configuration can go here
