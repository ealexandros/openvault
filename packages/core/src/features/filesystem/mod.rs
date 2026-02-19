mod codec;
mod error;
mod metadata;
mod records;
mod store;

pub use codec::{FilesystemCodec, FILESYSTEM_FEATURE_ID};
pub use error::{FilesystemError, Result};
pub use metadata::{
    FileMetadata, FileMetadataPatch, FolderMetadata, FolderMetadataPatch, ROOT_FOLDER_ID,
};
pub use records::{
    FilesystemChange, FilesystemDelta, FilesystemSnapshot, FILESYSTEM_WIRE_VERSION_V1,
};
pub use store::FilesystemStore;
