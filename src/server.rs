use crate::command::{CommandRegistry};
use crate::config::Config;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use std::error::Error;
use log::{info, error};
use std::sync::Arc;

// mod simulation;

async fn handle_client(stream: TcpStream, registry: Arc<CommandRegistry>) -> Result<(), Box<dyn Error>> {
    let mut buffered_stream = BufReader::new(stream);

    loop {
        let mut buffer_line = String::new();
        let bytes_read = buffered_stream.read_line(&mut buffer_line).await?;

        // If 0 bytes are read, the client has disconnected
        if bytes_read == 0 {
            break;
        }

        let mut parts = buffer_line.trim().splitn(2, ' ');
        let command_id_str = parts.next().ok_or("Missing command identifier")?;
        let command_data = parts.next().unwrap_or("");

        // Parse the command identifier
        let command_id = command_id_str.parse::<u16>().map_err(|_| "Invalid command identifier")?;

        // Dispatch the command based on the command_id
        if let Ok(command) = registry.create_command(command_id, command_data) {
            let mut response = command.execute();
            // Append a newline character to the response
            response.push('\n');

            // Get a mutable reference to the underlying TcpStream for writing and respond
            buffered_stream.get_mut().write_all(response.as_bytes()).await?;
        } else {
            return Err("Unknown command".into());
        }
    }

    Ok(())
}

pub async fn run_server(config: Config, registry: Arc<CommandRegistry>) {
    let address = format!("{}:{}", config.server.address, config.server.port);
    let listener = TcpListener::bind(&address).await.unwrap();

    info!("Server listening on {}", address);

    // Initialize the simulation state
    // let mut simulation = Simulation::new();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let registry_clone = Arc::clone(&registry);
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, registry_clone).await {
                error!("Error: {}", e);
            }
        });
    }
}
