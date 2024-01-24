use crate::command::{Command, HelloMessage};
use crate::config::Config;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use log::{info, error};

// mod simulation;

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;

    let received_data = String::from_utf8_lossy(&buffer);
    let command = serde_json::from_str::<HelloMessage>(&received_data)?;

    let response = command.execute();
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

pub async fn run_server(config: Config) {
    let address = format!("{}:{}", config.server.address, config.server.port);
    let listener = TcpListener::bind(&address).await.unwrap();

    info!("Server listening on {}", address);

    // Initialize the simulation state
    // let mut simulation = Simulation::new();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                error!("Error: {}", e);
            }
        });
    }
}
