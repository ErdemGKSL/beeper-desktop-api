//! Error types for Beeper API operations

use thiserror::Error;

/// Error type for Beeper API operations
#[derive(Error, Debug)]
pub enum BeeperError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API error: {message} (code: {code})")]
    ApiError { code: String, message: String },

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Result type for Beeper API operations
pub type Result<T> = std::result::Result<T, BeeperError>;
