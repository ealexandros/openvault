use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const ROOT_FOLDER_ID: Uuid = Uuid::nil();

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FolderMetadata {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FolderMetadata {
    pub fn new(id: Uuid, parent_id: Option<Uuid>, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id,
            parent_id,
            name: name.into(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn root() -> Self {
        Self::new(ROOT_FOLDER_ID, None, "/")
    }

    pub fn is_root(&self) -> bool {
        self.id == ROOT_FOLDER_ID
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FolderMetadataPatch {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl FolderMetadataPatch {
    pub fn new(updated_at: DateTime<Utc>) -> Self {
        Self {
            parent_id: None,
            name: None,
            updated_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetadata {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub name: String,
    pub size_bytes: u64,
    pub mime_type: Option<String>,
    pub content_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FileMetadata {
    pub fn new(id: Uuid, parent_id: Uuid, name: impl Into<String>, size_bytes: u64) -> Self {
        let now = Utc::now();
        Self {
            id,
            parent_id,
            name: name.into(),
            size_bytes,
            mime_type: None,
            content_hash: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetadataPatch {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub size_bytes: Option<u64>,
    pub mime_type: Option<Option<String>>,
    pub content_hash: Option<Option<String>>,
    pub updated_at: DateTime<Utc>,
}

impl FileMetadataPatch {
    pub fn new(updated_at: DateTime<Utc>) -> Self {
        Self {
            parent_id: None,
            name: None,
            size_bytes: None,
            mime_type: None,
            content_hash: None,
            updated_at,
        }
    }
}
