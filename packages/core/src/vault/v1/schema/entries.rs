use std::path::PathBuf;

use crate::crypto::compression::factory::CompressionAlgorithm;
use serde::{Deserialize, Serialize};

/// Metadata for a folder in the filesystem
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderMeta {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub deleted: bool,
}

/// Metadata for a file in the filesystem
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub blob: BlobRef,
    pub deleted: bool,
    #[serde(skip)]
    pub relative_path: PathBuf,
}

/// Reference to an encrypted blob in the vault
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobRef {
    pub offset: u64,
    pub size: u64,
    pub original_size: u64,
    pub compression: String,
}

/// Filesystem metadata containing files and folders
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FileSystemMeta {
    pub files: Vec<FileMeta>,
    pub folders: Vec<FolderMeta>,
}

/// Metadata for a secure note
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteMeta {
    pub id: u32,
    pub title: String,
    pub blob: BlobRef,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted: bool,
}

/// Metadata for an audit log entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogMeta {
    pub id: u32,
    pub event_type: u8,
    pub timestamp: u64,
    pub blob: BlobRef,
}

/// Metadata for a secret (password/TOTP) entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretMeta {
    pub id: u32,
    pub label: String,
    pub secret_type: SecretType,
    pub blob: BlobRef,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted: bool,
}

/// Type of secret stored
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum SecretType {
    #[default]
    Password,
    Totp,
    Both,
}

impl FolderMeta {
    pub fn new(id: u32, parent_id: Option<u32>, name: String) -> Self {
        Self {
            id,
            parent_id,
            name,
            deleted: false,
        }
    }

    pub fn root() -> Self {
        Self {
            id: 0,
            parent_id: None,
            name: "/".to_string(),
            deleted: false,
        }
    }
}

impl FileMeta {
    pub fn new(
        id: u32,
        parent_id: Option<u32>,
        name: String,
        size: u64,
        relative_path: PathBuf,
    ) -> Self {
        Self {
            id,
            parent_id,
            name,
            relative_path,
            deleted: false,
            blob: BlobRef {
                offset: 0,
                size: 0,
                original_size: size,
                compression: CompressionAlgorithm::default().to_string(),
            },
        }
    }
}
