use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FileSystemMeta {
    pub files: Vec<FileMeta>,
    pub folders: Vec<FolderMeta>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderMeta {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub blob: BlobRef,
    pub deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobRef {
    pub offset: u64,
    pub size: u64,
    pub original_size: u64,
    pub compression: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteMeta {
    pub id: u32,
    pub title: String,
    pub blob: BlobRef,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogMeta {
    pub id: u32,
    pub event_type: u8,
    pub timestamp: u64,
    pub blob: BlobRef,
}

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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum SecretType {
    #[default]
    Password,
    Totp,
    Both,
}
