mod codec;
mod error;
mod metadata;
mod records;
mod scanner;
mod store;

pub use codec::{FILESYSTEM_FEATURE_ID, FilesystemCodec};
pub use error::{FilesystemError, Result};
pub use metadata::{
    FileMetadata, FileMetadataPatch, FolderMetadata, FolderMetadataPatch, ROOT_FOLDER_ID,
};
pub use records::{
    FILESYSTEM_WIRE_VERSION_V1, FilesystemChange, FilesystemDelta, FilesystemSnapshot,
};
pub use store::FilesystemStore;

pub use scanner::scan_filesystem;
