use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Secret already exists: {0}")]
    AlreadyExists(String),

    #[error("Secret not found: {0}")]
    NotFound(String),

    #[error("Invalid key")]
    InvalidKey,
}

pub type Result<T = ()> = std::result::Result<T, SecretError>;
