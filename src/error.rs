use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Command execution failed: {0}")]
    Command(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Logging error: {0}")]
    Logging(String),
}

pub type Result<T> = std::result::Result<T, Error>;
