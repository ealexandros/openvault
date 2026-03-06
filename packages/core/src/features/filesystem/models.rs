use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use zeroize::Zeroize;

use crate::features::shared::BlobRef;

pub const ROOT_FOLDER_ID: Uuid = Uuid::nil();
pub const ROOT_FOLDER_NAME: &str = "/";
pub const DEFAULT_FOLDER_ICON: &str = "folder";

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
            icon: DEFAULT_FOLDER_ICON.to_string(),
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
            name: ROOT_FOLDER_NAME.to_string(),
            icon: DEFAULT_FOLDER_ICON.to_string(),
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

impl Zeroize for FolderMetadata {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.icon.zeroize();
    }
}

impl Zeroize for FileMetadata {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.extension.zeroize();
    }
}
