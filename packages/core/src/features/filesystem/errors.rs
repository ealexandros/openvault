use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FilesystemError {
    #[error("Invalid filesystem entry name: {0}")]
    InvalidName(String),

    #[error("Folder not found: {0}")]
    FolderNotFound(Uuid),

    #[error("File not found: {0}")]
    FileNotFound(Uuid),

    #[error("Parent folder not found: {0}")]
    ParentFolderNotFound(Uuid),

    #[error("Duplicate filesystem entry id: {0}")]
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

    #[error("Invalid folder move: {0}")]
    InvalidMove(String),

    #[error("Folder {0} is missing parent id")]
    FolderMissingParent(Uuid),

    #[error("Folder must have a parent {0}")]
    FolderMustHaveParent(Uuid),

    #[error("Cycle detected for folder {0}")]
    CycleDetected(Uuid),

    #[error("Unsupported filesystem wire version: {0}")]
    UnsupportedWireVersion(u16),

    #[error("Invalid filesystem payload: {0}")]
    InvalidPayload(String),

    #[error("Invalid snapshot")]
    InvalidSnapshot,

    #[error("Name exhausted under folder {parent_id} for entry {name}")]
    NameExhausted { parent_id: Uuid, name: String },
}

impl FilesystemError {
    pub fn name_conflict(parent_id: Uuid, name: &str) -> Self {
        Self::NameConflict {
            parent_id,
            name: name.to_string(),
        }
    }

    pub fn name_exhausted(parent_id: Uuid, name: &str) -> Self {
        Self::NameExhausted {
            parent_id,
            name: name.to_string(),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, FilesystemError>;
