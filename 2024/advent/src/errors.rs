use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Failed to read lines from file")]
    ReadError,
}