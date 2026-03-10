use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum MessagesError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Duplicate message contact id: {0}")]
    DuplicateId(Uuid),

    #[error("Message contact not found: {0}")]
    NotFound(String),

    #[error("Unsupported messages wire version: {0}")]
    UnsupportedWireVersion(u16),

    #[error("Invalid messages payload: {0}")]
    InvalidPayload(String),

    #[error("Invalid snapshot")]
    InvalidSnapshot,

    #[error(transparent)]
    Crypto(#[from] openvault_crypto::errors::Error),

    #[error("Credentials have not been set")]
    CredentialsNotSet,
}

pub type Result<T = ()> = std::result::Result<T, MessagesError>;
