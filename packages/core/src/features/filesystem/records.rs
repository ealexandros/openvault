use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::metadata::{FileMetadata, FileMetadataPatch, FolderMetadata, FolderMetadataPatch};

pub const FILESYSTEM_WIRE_VERSION_V1: u16 = 1;

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
    FolderUpdated {
        id: Uuid,
        patch: FolderMetadataPatch,
    },
    FolderDeleted {
        id: Uuid,
    },
    FileAdded(FileMetadata),
    FileUpdated {
        id: Uuid,
        patch: FileMetadataPatch,
    },
    FileDeleted {
        id: Uuid,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilesystemChange {
    Snapshot(FilesystemSnapshot),
    Deltas(Vec<FilesystemDelta>),
}
