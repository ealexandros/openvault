use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::filesystem::FilesystemError;

use super::models::{FileMetadata, FolderMetadata};
use super::patch::{FileMetadataPatch, FolderMetadataPatch};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct FilesystemSnapshot {
    pub folders: HashMap<Uuid, FolderMetadata>,
    pub files: HashMap<Uuid, FileMetadata>,
}

impl FilesystemSnapshot {
    pub fn new(folders: HashMap<Uuid, FolderMetadata>, files: HashMap<Uuid, FileMetadata>) -> Self {
        Self { folders, files }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilesystemDelta {
    FolderAdded(FolderMetadata),
    FolderDeleted(Uuid),
    FolderUpdated {
        id: Uuid,
        patch: FolderMetadataPatch,
    },
    FileAdded(FileMetadata),
    FileDeleted(Uuid),
    FileUpdated {
        id: Uuid,
        patch: FileMetadataPatch,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilesystemChange {
    Snapshot(FilesystemSnapshot),
    Deltas(Vec<FilesystemDelta>),
}

impl TryFrom<FilesystemChange> for FilesystemSnapshot {
    type Error = FilesystemError;

    fn try_from(value: FilesystemChange) -> Result<Self, Self::Error> {
        match value {
            FilesystemChange::Snapshot(snapshot) => Ok(snapshot),
            FilesystemChange::Deltas(_) => Err(FilesystemError::InvalidSnapshot),
        }
    }
}
