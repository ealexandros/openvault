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

    #[error("Folder {0} is not empty")]
    FolderNotEmpty(Uuid),

    #[error("Invalid folder move: {0}")]
    InvalidMove(String),

    #[error("Unsupported filesystem wire version: {0}")]
    UnsupportedWireVersion(u16),

    #[error("Invalid filesystem payload: {0}")]
    InvalidPayload(String),
}

pub type Result<T = ()> = std::result::Result<T, FilesystemError>;
