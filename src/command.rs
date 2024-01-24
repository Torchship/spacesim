use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

pub struct CommandRegistry {
    commands: HashMap<u16, fn(&str) -> Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> CommandRegistry {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: u16, command: fn(&str) -> Box<dyn Command>) {
        self.commands.insert(id, command);
    }

    pub fn create_command(&self, id: u16, data: &str) -> Result<Box<dyn Command>, &'static str> {
        if let Some(&constructor) = self.commands.get(&id) {
            Ok(constructor(data))
        } else {
            Err("Unknown command")
        }
    }
}

#[derive(Deserialize)]
pub struct HelloMessage {
    message: String,
}

#[derive(Serialize)]
struct AckMessage {
    ack: bool,
}

pub trait Command: Send  {
    fn execute(&self) -> String;
}

impl Command for HelloMessage {
    fn execute(&self) -> String {
        serde_json::to_string(&AckMessage { ack: true }).unwrap()
    }
}