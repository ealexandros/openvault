use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SecretError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Folder not found: {0}")]
    FolderNotFound(Uuid),

    #[error("Entry not found: {0}")]
    EntryNotFound(Uuid),

    #[error("Parent folder not found: {0}")]
    ParentFolderNotFound(Uuid),

    #[error("Duplicate secret id: {0}")]
    DuplicateId(Uuid),

    #[error("Name conflict under folder {parent_id} for entry {name}")]
    NameConflict { parent_id: Uuid, name: String },

    #[error("Cannot delete root folder")]
    CannotDeleteRootFolder,

    #[error("Invalid root folder state: {0}")]
    RootFolderInvariant(String),

    #[error("Root folder metadata is reserved")]
    RootFolderReserved,

    #[error("Root folder cannot be modified")]
    RootFolderImmutable,

    #[error("Root folder must not have a parent")]
    RootFolderMustNotHaveParent,

    #[error("Root folder must have name '/'")]
    RootFolderMustHaveName,

    #[error("Folder {0} is missing parent id")]
    FolderMissingParent(Uuid),

    #[error("Folder must have a parent {0}")]
    FolderMustHaveParent(Uuid),

    #[error("Cycle detected for folder {0}")]
    CycleDetected(Uuid),

    #[error("Unsupported secrets wire version: {0}")]
    UnsupportedWireVersion(u16),

    #[error("Invalid secrets payload: {0}")]
    InvalidPayload(String),

    #[error("Invalid snapshot")]
    InvalidSnapshot,

    #[error("Cryptography error: {0}")]
    CryptoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl SecretError {
    pub fn name_conflict(parent_id: Uuid, name: &str) -> Self {
        Self::NameConflict {
            parent_id,
            name: name.to_string(),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, SecretError>;
