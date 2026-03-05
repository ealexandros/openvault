pub mod errors;
pub mod store;

mod codec;
mod events;
mod index;
mod models;
mod namings;
mod patch;
mod validate;

pub use codec::{FILESYSTEM_WIRE_VERSION, FilesystemCodec};
pub use errors::{FilesystemError, Result};
pub use events::{FilesystemChange, FilesystemDelta, FilesystemSnapshot};
pub use models::{FileMetadata, FolderMetadata, ROOT_FOLDER_ID};
pub use patch::{FileMetadataPatch, FolderMetadataPatch};
pub use store::FilesystemStore;

pub use crate::internal::scanner::{ScannedFolder, scan_directory};
