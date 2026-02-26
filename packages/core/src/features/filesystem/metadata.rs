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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, Eq)]
pub struct FolderMetadataPatch {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl FolderMetadataPatch {
    pub fn default() -> Self {
        Self {
            parent_id: None,
            name: None,
            updated_at: Utc::now(),
        }
    }

    pub fn rename(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn move_to(parent_id: Uuid) -> Self {
        Self {
            parent_id: Some(parent_id),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetadata {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub name: String,
    pub extension: String,
    pub blob: BlobRef,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FileMetadata {
    pub fn new(
        parent_id: Uuid,
        name: impl Into<String>,
        extension: impl Into<String>,
        blob: BlobRef,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            parent_id,
            name: name.into(),
            extension: extension.into(),
            blob,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn size_bytes(&self) -> u64 {
        self.blob.size_bytes
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, Eq)]
pub struct FileMetadataPatch {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub extension: Option<String>,
    pub blob: Option<BlobRef>,
    pub updated_at: DateTime<Utc>,
}

impl FileMetadataPatch {
    pub fn default() -> Self {
        Self {
            parent_id: None,
            name: None,
            extension: None,
            blob: None,
            updated_at: Utc::now(),
        }
    }

    pub fn rename(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn move_to(parent_id: Uuid) -> Self {
        Self {
            parent_id: Some(parent_id),
            ..Default::default()
        }
    }
}
