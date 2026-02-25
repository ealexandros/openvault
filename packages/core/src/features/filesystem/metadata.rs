use crate::features::shared::blob_ref::BlobRef;
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
    pub fn new(parent_id: Option<Uuid>, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            parent_id,
            name: name.into(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn root() -> Self {
        let now = Utc::now();
        Self {
            id: ROOT_FOLDER_ID,
            parent_id: None,
            name: "/".to_string(),
            created_at: now,
            updated_at: now,
        }
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
    pub mime_type: Option<String>,
    pub blob: Option<BlobRef>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FileMetadata {
    pub fn new(parent_id: Uuid, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            parent_id,
            name: name.into(),
            mime_type: None,
            blob: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn size_bytes(&self) -> u64 {
        self.blob.as_ref().map(|b| b.size_bytes).unwrap_or(0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetadataPatch {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub mime_type: Option<Option<String>>,
    pub blob: Option<Option<BlobRef>>,
    pub updated_at: DateTime<Utc>,
}

impl FileMetadataPatch {
    pub fn new(updated_at: DateTime<Utc>) -> Self {
        Self {
            parent_id: None,
            name: None,
            mime_type: None,
            blob: None,
            updated_at,
        }
    }
}
