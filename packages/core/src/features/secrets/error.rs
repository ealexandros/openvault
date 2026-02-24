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

    #[error("Duplicate secret ID: {0}")]
    DuplicateId(uuid::Uuid),

    #[error("Secret not found: {0}")]
    NotFound(String),

    #[error("Invalid key")]
    InvalidKey,

    #[error("Unsupported secrets wire version: {0}")]
    UnsupportedWireVersion(u16),

    #[error("Invalid feature record kind: expected {expected:?}, actual {actual:?}")]
    InvalidRecordKind {
        expected: crate::features::feature_trait::RecordKind,
        actual: crate::features::feature_trait::RecordKind,
    },
}

pub type Result<T = ()> = std::result::Result<T, SecretError>;
