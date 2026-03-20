pub mod store;

mod codec;
mod errors;
mod events;
mod index;
mod models;
mod namings;
mod patch;
mod validate;

#[cfg(test)]
mod tests;

pub use codec::{FILESYSTEM_WIRE_VERSION, FilesystemCodec};
pub use errors::{FilesystemError, Result};
pub use events::{FilesystemChange, FilesystemDelta, FilesystemSnapshot};
pub use models::{FILESYSTEM_ROOT_FOLDER_ID, FileMetadata, FolderMetadata};
pub use patch::{FileMetadataPatch, FolderMetadataPatch};
pub use store::FilesystemStore;

pub use crate::internal::scanner::{ScannedFolder, scan_directory};
