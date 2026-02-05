use crate::constants::NONCE_LEN;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SectionIndex<T> {
    pub entries: Vec<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderEntry {
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub offset: u64,
    pub compressed_size: u64,
    pub original_size: u64,
    pub nonce: [u8; NONCE_LEN],
    pub compression: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteEntry {
    pub offset: u64,
    pub len: u32,
    pub nonce: [u8; NONCE_LEN],
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub offset: u64,
    pub len: u32,
    pub nonce: [u8; NONCE_LEN],
    pub timestamp: u64,
    pub event_type: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IndexEntry {
    File(FileEntry),
    Folder(FolderEntry),
}

impl<T> Default for SectionIndex<T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl FolderEntry {
    pub fn root() -> Self {
        Self {
            path: "/".to_string(),
        }
    }
    pub fn new(path: String) -> Self {
        Self { path }
    }
}
