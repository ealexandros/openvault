use std::collections::HashMap;

use uuid::Uuid;

use super::error::{FilesystemError, Result};
use super::metadata::{
    FileMetadata, FileMetadataPatch, FolderMetadata, FolderMetadataPatch, ROOT_FOLDER_ID,
};
use super::records::{FilesystemChange, FilesystemDelta, FilesystemSnapshot};

mod mutations;
mod validation;

const SNAPSHOT_THRESHOLD: usize = 64;

#[derive(Clone, Debug)]
pub struct FilesystemStore {
    folders: HashMap<Uuid, FolderMetadata>,
    files: HashMap<Uuid, FileMetadata>,
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
            deltas: Vec::new(),
        }
    }

    pub fn restore(snapshot: FilesystemSnapshot, deltas: Vec<FilesystemDelta>) -> Result<Self> {
        let mut store = Self {
            folders: snapshot.folders,
            files: snapshot.files,
            deltas: Vec::new(),
        };

        store
            .folders
            .entry(ROOT_FOLDER_ID)
            .or_insert_with(FolderMetadata::root);

        store.validate_snapshot()?;

        for delta in &deltas {
            store.apply_delta_without_tracking(delta)?;
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
        let mut folders: Vec<FolderMetadata> = self
            .folders
            .values()
            .filter(|f| f.parent_id == Some(parent_id))
            .cloned()
            .collect();

        folders.sort_by(|a, b| a.name.cmp(&b.name));

        folders
    }

    pub fn files(&self, parent_id: Uuid) -> Vec<FileMetadata> {
        let mut files: Vec<FileMetadata> = self
            .files
            .values()
            .filter(|f| f.parent_id == parent_id)
            .cloned()
            .collect();

        files.sort_by(|a, b| a.name.cmp(&b.name));

        files
    }

    pub fn browse(&self, parent_id: &Uuid) -> Result<(Vec<FolderMetadata>, Vec<FileMetadata>)> {
        if !self.folders.contains_key(parent_id) {
            return Err(FilesystemError::FolderNotFound(*parent_id));
        }

        Ok((self.folders(*parent_id), self.files(*parent_id)))
    }

    pub fn add_folder(&mut self, parent_id: Uuid, name: String) -> Result<Uuid> {
        let folder = FolderMetadata::new(Some(parent_id), name);
        let id = folder.id;

        self.apply_delta_with_tracking(&mut FilesystemDelta::FolderAdded(folder))?;

        Ok(id)
    }

    pub fn remove_folder(&mut self, id: Uuid) -> Result {
        self.apply_delta_with_tracking(&FilesystemDelta::FolderDeleted { id })
    }

    pub fn add_file(
        &mut self,
        parent_id: Uuid,
        name: String,
        mime_type: String,
        blob: crate::features::shared::blob_ref::BlobRef,
    ) -> Result<Uuid> {
        let file = FileMetadata::new(parent_id, name, mime_type, blob);
        let id = file.id;

        self.apply_delta_with_tracking(&FilesystemDelta::FileAdded(file))?;

        Ok(id)
    }

    pub fn remove_file(&mut self, id: Uuid) -> Result {
        self.apply_delta_with_tracking(&FilesystemDelta::FileDeleted { id })
    }

    pub fn patch_file(&mut self, id: Uuid, patch: FileMetadataPatch) -> Result {
        self.apply_delta_with_tracking(&FilesystemDelta::FileUpdated { id, patch })
    }

    pub fn patch_folder(&mut self, id: Uuid, patch: FolderMetadataPatch) -> Result {
        self.apply_delta_with_tracking(&FilesystemDelta::FolderUpdated { id, patch })
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
