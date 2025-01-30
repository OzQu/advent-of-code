use thiserror::Error;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Failed to read lines from file")]
    ReadError,
    #[error("Failed to parse integer: {0}")]
    ParseIntError(#[from] ParseIntError),
}