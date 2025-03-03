use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Command: Send + Sync {
    async fn execute(&self) -> Result<()>;
}

pub struct CommandRegistry {
    commands: std::collections::HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, command: Box<dyn Command>) {
        self.commands.insert(name.to_string(), command);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Command> {
        self.commands.get(name).map(|cmd| cmd.as_ref())
    }
}

// Re-export command implementations
pub mod explain;
