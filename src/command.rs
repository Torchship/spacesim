use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize)]
pub struct HelloMessage {
    message: String,
}

#[derive(Serialize)]
struct AckMessage {
    ack: bool,
}

pub trait Command {
    fn execute(&self) -> String;
}

impl Command for HelloMessage {
    fn execute(&self) -> String {
        serde_json::to_string(&AckMessage { ack: true }).unwrap()
    }
}