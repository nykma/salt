use thiserror::Error;

#[derive(Error, Debug)]
pub enum SaltError {
    #[error("Hashing error: {0}")]
    HashingError(String),

    #[error("Verification error: {0}")]
    VerificationError(String),

    #[error("Invalid algorithm: {0}")]
    InvalidAlgorithm(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, SaltError>;
