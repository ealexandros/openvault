use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::features::shared::blob_ref::BlobRef;

pub const ROOT_FOLDER_ID: Uuid = Uuid::nil();

// @todo-soon include validation using validator..
// @todo-soon rethink about sanitize_name, currently not being used

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct FolderMetadata {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    #[validate(length(min = 1, max = 255))]
    #[validate(custom(function = "super::validate::validate_safe_name"))]
    pub name: String,
    #[validate(length(max = 50))]
    pub icon: String,
    pub is_favourite: bool,
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
            icon: "folder".to_string(),
            is_favourite: false,
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
            icon: "folder".to_string(),
            is_favourite: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct FileMetadata {
    pub id: Uuid,
    pub parent_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    #[validate(custom(function = "super::validate::validate_safe_name"))]
    pub name: String,
    #[validate(length(max = 10))]
    pub extension: String,
    pub blob: BlobRef,
    pub is_favourite: bool,
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
            is_favourite: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn size_bytes(&self) -> u64 {
        self.blob.size_bytes
    }
}
