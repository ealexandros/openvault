use std::collections::HashMap;

use uuid::Uuid;

use crate::features::shared::blob_ref::BlobRef;

use super::error::{FilesystemError, Result};
use super::metadata::{FileMetadata, FileMetadataPatch, FolderMetadata, FolderMetadataPatch};
use super::records::{FilesystemChange, FilesystemDelta, FilesystemSnapshot};

mod index;
mod mutations;
mod validation;

use index::FilesystemIndex;

const SNAPSHOT_THRESHOLD: usize = 64;

#[derive(Clone, Debug)]
pub struct FilesystemStore {
    folders: HashMap<Uuid, FolderMetadata>,
    files: HashMap<Uuid, FileMetadata>,
    index: FilesystemIndex,
    deltas: Vec<FilesystemDelta>,
}

impl Default for FilesystemStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FilesystemStore {
    pub fn new() -> Self {
        let root = FolderMetadata::root();
        let mut folders = HashMap::new();
        folders.insert(root.id, root);

        Self {
            folders,
            files: HashMap::new(),
            index: FilesystemIndex::new(),
            deltas: Vec::new(),
        }
    }

    pub fn restore(snapshot: FilesystemSnapshot, deltas: Vec<FilesystemDelta>) -> Result<Self> {
        let index = FilesystemIndex::build(&snapshot.folders, &snapshot.files);

        let mut store = Self {
            folders: snapshot.folders,
            files: snapshot.files,
            index,
            deltas: Vec::new(),
        };

        store.validate_snapshot()?;

        for delta in &deltas {
            store.replay_delta(delta)?;
        }

        store.clear_deltas();

        Ok(store)
    }

    pub fn folder(&self, id: &Uuid) -> Option<&FolderMetadata> {
        self.folders.get(id)
    }

    pub fn file(&self, id: &Uuid) -> Option<&FileMetadata> {
        self.files.get(id)
    }

    pub fn folders(&self, parent_id: Uuid) -> Vec<FolderMetadata> {
        let folder_ids = self.index.folder_children(&parent_id);

        folder_ids
            .iter()
            .filter_map(|id| self.folders.get(id))
            .cloned()
            .collect()
    }

    pub fn files(&self, parent_id: Uuid) -> Vec<FileMetadata> {
        let file_ids = self.index.file_children(&parent_id);

        file_ids
            .iter()
            .filter_map(|id| self.files.get(id))
            .cloned()
            .collect()
    }

    pub fn browse(&self, parent_id: &Uuid) -> Result<(Vec<FolderMetadata>, Vec<FileMetadata>)> {
        if !self.folders.contains_key(parent_id) {
            return Err(FilesystemError::FolderNotFound(*parent_id));
        }

        Ok((self.folders(*parent_id), self.files(*parent_id)))
    }

    pub fn file_exists_in_folder(&self, parent_id: &Uuid, name: &str) -> bool {
        let file_ids = self.index.file_children(parent_id);

        file_ids
            .iter()
            .filter_map(|id| self.files.get(id))
            .any(|f| f.name == name)
    }

    pub fn children_count(&self, parent_id: &Uuid) -> usize {
        self.index.children_count(parent_id)
    }

    pub fn add_folder(&mut self, parent_id: Uuid, name: String) -> Result<Uuid> {
        let folder = FolderMetadata::new(Some(parent_id), name);
        let id = folder.id;

        self.commit_delta(&mut FilesystemDelta::FolderAdded(folder))?;

        Ok(id)
    }

    pub fn add_file(
        &mut self,
        parent_id: Uuid,
        name: String,
        extension: String,
        blob: BlobRef,
    ) -> Result<Uuid> {
        let file = FileMetadata::new(parent_id, name, extension, blob);
        let id = file.id;

        self.commit_delta(&FilesystemDelta::FileAdded(file))?;

        Ok(id)
    }

    pub fn remove_file(&mut self, id: Uuid) -> Result {
        self.commit_delta(&FilesystemDelta::FileDeleted { id })
    }

    pub fn remove_folder(&mut self, id: Uuid) -> Result {
        self.commit_delta(&FilesystemDelta::FolderDeleted { id })
    }

    pub fn rename_file(&mut self, id: Uuid, new_name: String) -> Result {
        let patch = FileMetadataPatch::rename(new_name);
        self.commit_delta(&FilesystemDelta::FileUpdated { id, patch })
    }

    pub fn rename_folder(&mut self, id: Uuid, new_name: String) -> Result {
        let patch = FolderMetadataPatch::rename(new_name);
        self.commit_delta(&FilesystemDelta::FolderUpdated { id, patch })
    }

    pub fn move_file(&mut self, id: Uuid, new_parent_id: Uuid) -> Result {
        let patch = FileMetadataPatch::move_to(new_parent_id);
        self.commit_delta(&FilesystemDelta::FileUpdated { id, patch })
    }

    pub fn move_folder(&mut self, id: Uuid, new_parent_id: Uuid) -> Result {
        let patch = FolderMetadataPatch::move_to(new_parent_id);
        self.commit_delta(&FilesystemDelta::FolderUpdated { id, patch })
    }

    pub fn snapshot(&self) -> FilesystemSnapshot {
        FilesystemSnapshot::new(self.folders.clone(), self.files.clone())
    }

    pub fn pending_changes(&self) -> Option<FilesystemChange> {
        if self.deltas.is_empty() {
            return None;
        }

        if self.deltas.len() >= SNAPSHOT_THRESHOLD {
            return Some(FilesystemChange::Snapshot(self.snapshot()));
        }

        Some(FilesystemChange::Deltas(self.deltas.clone()))
    }

    pub fn clear_deltas(&mut self) {
        self.deltas.clear();
    }
}
