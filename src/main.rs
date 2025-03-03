use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, FmtSubscriber};

mod commands;
mod error;

use crate::error::Result;

#[derive(Parser, Debug)]
#[command(author, version, about = "Groundhog AI coding assistant", long_about = None)]
struct Args {
    /// Set the logging level (debug, info, warn, error)
    #[arg(short, long, default_value = "info", value_enum)]
    log_level: LogLevel,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Explain the given text
    Explain {
        /// The text to explain
        text: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(args.log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting Groundhog...");

    match args.command {
        Some(Commands::Explain { text }) => {
            info!("Explaining text: {}", text);
            // TODO: Implement explain command
            println!("Hello, world!");
        }
        None => {
            println!("Hello, world!");
        }
    }

    Ok(())
}
