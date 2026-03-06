use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

use crate::features::shared::BlobRef;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetadataPatch {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub extension: Option<String>,
    pub blob: Option<BlobRef>,
    pub is_favourite: Option<bool>,
    pub updated_at: DateTime<Utc>,
}

impl Default for FileMetadataPatch {
    fn default() -> Self {
        Self {
            parent_id: None,
            name: None,
            extension: None,
            blob: None,
            is_favourite: None,
            updated_at: Utc::now(),
        }
    }
}

impl FileMetadataPatch {
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

    pub fn set_favourite(is_favourite: bool) -> Self {
        Self {
            is_favourite: Some(is_favourite),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FolderMetadataPatch {
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub is_favourite: Option<bool>,
    pub updated_at: DateTime<Utc>,
}

impl Default for FolderMetadataPatch {
    fn default() -> Self {
        Self {
            parent_id: None,
            name: None,
            icon: None,
            is_favourite: None,
            updated_at: Utc::now(),
        }
    }
}

impl FolderMetadataPatch {
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

    pub fn set_icon(icon: impl Into<String>) -> Self {
        Self {
            icon: Some(icon.into()),
            ..Default::default()
        }
    }

    pub fn set_favourite(favourite: bool) -> Self {
        Self {
            is_favourite: Some(favourite),
            ..Default::default()
        }
    }
}

impl Zeroize for FileMetadataPatch {
    fn zeroize(&mut self) {
        if let Some(name) = &mut self.name {
            name.zeroize();
        }
        if let Some(extension) = &mut self.extension {
            extension.zeroize();
        }
    }
}

impl Zeroize for FolderMetadataPatch {
    fn zeroize(&mut self) {
        if let Some(name) = &mut self.name {
            name.zeroize();
        }
        if let Some(icon) = &mut self.icon {
            icon.zeroize();
        }
    }
}
