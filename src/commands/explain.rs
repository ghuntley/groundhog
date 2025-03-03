use crate::commands::Command;
use crate::error::Result;
use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
pub struct ExplainCommand {
    /// The text to explain
    #[arg(required = true)]
    text: String,
}

impl ExplainCommand {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

#[async_trait::async_trait]
impl Command for ExplainCommand {
    async fn execute(&self) -> Result<()> {
        info!("Executing explain command for text: {}", self.text);
        println!("Hello world");
        Ok(())
    }
}
