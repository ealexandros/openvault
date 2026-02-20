use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use super::error::{FilesystemError, Result};
use super::metadata::{
    FileMetadata, FileMetadataPatch, FolderMetadata, FolderMetadataPatch, ROOT_FOLDER_ID,
};
use super::records::{FilesystemChange, FilesystemDelta, FilesystemSnapshot};

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

        if !store.folders.contains_key(&ROOT_FOLDER_ID) {
            store.folders.insert(ROOT_FOLDER_ID, FolderMetadata::root());
        }

        store.validate_snapshot()?;

        for delta in &deltas {
            store.apply_delta(delta, false)?;
        }

        store.reset_sync_state();
        Ok(store)
    }

    pub fn folder(&self, id: &Uuid) -> Option<&FolderMetadata> {
        self.folders.get(id)
    }

    pub fn file(&self, id: &Uuid) -> Option<&FileMetadata> {
        self.files.get(id)
    }

    pub fn folders(&self) -> Vec<FolderMetadata> {
        let mut folders: Vec<FolderMetadata> = self.folders.values().cloned().collect();
        folders.sort_by(|a, b| {
            a.parent_id
                .unwrap_or(ROOT_FOLDER_ID)
                .cmp(&b.parent_id.unwrap_or(ROOT_FOLDER_ID))
                .then_with(|| a.name.cmp(&b.name))
                .then_with(|| a.id.cmp(&b.id))
        });
        folders
    }

    pub fn files(&self) -> Vec<FileMetadata> {
        let mut files: Vec<FileMetadata> = self.files.values().cloned().collect();
        files.sort_by(|a, b| {
            a.parent_id
                .cmp(&b.parent_id)
                .then_with(|| a.name.cmp(&b.name))
                .then_with(|| a.id.cmp(&b.id))
        });
        files
    }

    pub fn snapshot(&self) -> FilesystemSnapshot {
        FilesystemSnapshot::new(self.folders.clone(), self.files.clone())
    }

    pub fn create_snapshot(&self) -> FilesystemChange {
        FilesystemChange::Snapshot(self.snapshot())
    }

    pub fn pending_changes(&self) -> Option<FilesystemChange> {
        if self.deltas.is_empty() {
            return None;
        }

        if self.deltas.len() >= SNAPSHOT_THRESHOLD {
            return Some(self.create_snapshot());
        }

        Some(FilesystemChange::Deltas(self.deltas.clone()))
    }

    pub fn reset_sync_state(&mut self) {
        self.deltas.clear();
    }

    pub fn apply_change(&mut self, change: FilesystemChange) -> Result {
        match change {
            FilesystemChange::Snapshot(snapshot) => self.replace_snapshot(snapshot)?,
            FilesystemChange::Deltas(deltas) => {
                for delta in &deltas {
                    self.apply_delta(delta, false)?;
                }
            }
        }

        self.reset_sync_state();
        Ok(())
    }

    pub fn add_folder(&mut self, folder: FolderMetadata) -> Result<Uuid> {
        let id = folder.id;
        self.apply_delta(&FilesystemDelta::FolderAdded(folder), true)?;
        Ok(id)
    }

    pub fn update_folder(&mut self, id: Uuid, patch: FolderMetadataPatch) -> Result {
        self.apply_delta(&FilesystemDelta::FolderUpdated { id, patch }, true)
    }

    pub fn delete_folder(&mut self, id: Uuid) -> Result {
        self.apply_delta(&FilesystemDelta::FolderDeleted { id }, true)
    }

    pub fn add_file(&mut self, file: FileMetadata) -> Result<Uuid> {
        let id = file.id;
        self.apply_delta(&FilesystemDelta::FileAdded(file), true)?;
        Ok(id)
    }

    pub fn update_file(&mut self, id: Uuid, patch: FileMetadataPatch) -> Result {
        self.apply_delta(&FilesystemDelta::FileUpdated { id, patch }, true)
    }

    pub fn delete_file(&mut self, id: Uuid) -> Result {
        self.apply_delta(&FilesystemDelta::FileDeleted { id }, true)
    }

    fn replace_snapshot(&mut self, snapshot: FilesystemSnapshot) -> Result {
        self.folders = snapshot.folders;
        self.files = snapshot.files;

        if !self.folders.contains_key(&ROOT_FOLDER_ID) {
            self.folders.insert(ROOT_FOLDER_ID, FolderMetadata::root());
        }

        self.validate_snapshot()
    }

    fn apply_delta(&mut self, delta: &FilesystemDelta, track_delta: bool) -> Result {
        match delta {
            FilesystemDelta::FolderAdded(folder) => self.insert_folder(folder.clone())?,
            FilesystemDelta::FolderUpdated { id, patch } => {
                self.patch_folder(*id, patch.clone())?
            }
            FilesystemDelta::FolderDeleted { id } => self.remove_folder(*id)?,
            FilesystemDelta::FileAdded(file) => self.insert_file(file.clone())?,
            FilesystemDelta::FileUpdated { id, patch } => self.patch_file(*id, patch.clone())?,
            FilesystemDelta::FileDeleted { id } => self.remove_file(*id)?,
        }

        if track_delta {
            self.deltas.push(delta.clone());
        }

        Ok(())
    }

    fn insert_folder(&mut self, folder: FolderMetadata) -> Result {
        if folder.id == ROOT_FOLDER_ID {
            return Err(FilesystemError::RootFolderInvariant(
                "root folder metadata is reserved".to_string(),
            ));
        }

        if self.folders.contains_key(&folder.id) || self.files.contains_key(&folder.id) {
            return Err(FilesystemError::DuplicateId(folder.id));
        }

        let parent_id = folder.parent_id.ok_or_else(|| {
            FilesystemError::InvalidMove(format!("folder {} must have a parent", folder.id))
        })?;

        if !self.folders.contains_key(&parent_id) {
            return Err(FilesystemError::ParentFolderNotFound(parent_id));
        }

        let normalized_name = normalize_entry_name(&folder.name)?;
        self.ensure_name_available(parent_id, &normalized_name, None, None)?;

        let mut folder = folder;
        folder.name = normalized_name;
        self.folders.insert(folder.id, folder);
        Ok(())
    }

    fn patch_folder(&mut self, id: Uuid, patch: FolderMetadataPatch) -> Result {
        if id == ROOT_FOLDER_ID {
            return Err(FilesystemError::RootFolderInvariant(
                "root folder metadata cannot be modified".to_string(),
            ));
        }

        let current = self
            .folders
            .get(&id)
            .cloned()
            .ok_or(FilesystemError::FolderNotFound(id))?;

        let current_parent = current.parent_id.ok_or_else(|| {
            FilesystemError::RootFolderInvariant(format!(
                "folder {} is missing parent metadata",
                id
            ))
        })?;

        let target_parent = patch.parent_id.unwrap_or(current_parent);
        if !self.folders.contains_key(&target_parent) {
            return Err(FilesystemError::ParentFolderNotFound(target_parent));
        }

        if target_parent == id || self.would_create_cycle(id, target_parent) {
            return Err(FilesystemError::InvalidMove(format!(
                "folder {} cannot be moved under itself or one of its descendants",
                id
            )));
        }

        let target_name = patch
            .name
            .as_deref()
            .map(normalize_entry_name)
            .transpose()?
            .unwrap_or(current.name);

        self.ensure_name_available(target_parent, &target_name, Some(id), None)?;

        let folder = self
            .folders
            .get_mut(&id)
            .ok_or(FilesystemError::FolderNotFound(id))?;
        folder.parent_id = Some(target_parent);
        folder.name = target_name;
        folder.updated_at = patch.updated_at;
        Ok(())
    }

    fn remove_folder(&mut self, id: Uuid) -> Result {
        if id == ROOT_FOLDER_ID {
            return Err(FilesystemError::CannotDeleteRootFolder);
        }

        if !self.folders.contains_key(&id) {
            return Err(FilesystemError::FolderNotFound(id));
        }

        let has_folder_children = self
            .folders
            .values()
            .any(|folder| folder.parent_id == Some(id));
        let has_file_children = self.files.values().any(|file| file.parent_id == id);

        if has_folder_children || has_file_children {
            return Err(FilesystemError::FolderNotEmpty(id));
        }

        self.folders.remove(&id);
        Ok(())
    }

    fn insert_file(&mut self, file: FileMetadata) -> Result {
        if self.files.contains_key(&file.id) || self.folders.contains_key(&file.id) {
            return Err(FilesystemError::DuplicateId(file.id));
        }

        if !self.folders.contains_key(&file.parent_id) {
            return Err(FilesystemError::ParentFolderNotFound(file.parent_id));
        }

        let normalized_name = normalize_entry_name(&file.name)?;
        self.ensure_name_available(file.parent_id, &normalized_name, None, None)?;

        let mut file = file;
        file.name = normalized_name;
        self.files.insert(file.id, file);
        Ok(())
    }

    fn patch_file(&mut self, id: Uuid, patch: FileMetadataPatch) -> Result {
        let current = self
            .files
            .get(&id)
            .cloned()
            .ok_or(FilesystemError::FileNotFound(id))?;

        let target_parent = patch.parent_id.unwrap_or(current.parent_id);
        if !self.folders.contains_key(&target_parent) {
            return Err(FilesystemError::ParentFolderNotFound(target_parent));
        }

        let target_name = patch
            .name
            .as_deref()
            .map(normalize_entry_name)
            .transpose()?
            .unwrap_or(current.name);

        self.ensure_name_available(target_parent, &target_name, None, Some(id))?;

        let file = self
            .files
            .get_mut(&id)
            .ok_or(FilesystemError::FileNotFound(id))?;

        file.parent_id = target_parent;
        file.name = target_name;
        file.updated_at = patch.updated_at;

        if let Some(mime_type) = patch.mime_type {
            file.mime_type = mime_type;
        }
        if let Some(blob) = patch.blob {
            file.blob = blob;
        }

        Ok(())
    }

    fn remove_file(&mut self, id: Uuid) -> Result {
        if self.files.remove(&id).is_none() {
            return Err(FilesystemError::FileNotFound(id));
        }
        Ok(())
    }

    fn validate_snapshot(&self) -> Result {
        let root = self.folders.get(&ROOT_FOLDER_ID).ok_or_else(|| {
            FilesystemError::RootFolderInvariant("root folder is missing".to_string())
        })?;

        if root.parent_id.is_some() {
            return Err(FilesystemError::RootFolderInvariant(
                "root folder must not have a parent".to_string(),
            ));
        }

        if root.name != "/" {
            return Err(FilesystemError::RootFolderInvariant(
                "root folder name must be '/'".to_string(),
            ));
        }

        for folder in self.folders.values() {
            if folder.id == ROOT_FOLDER_ID {
                continue;
            }

            let parent_id = folder.parent_id.ok_or_else(|| {
                FilesystemError::InvalidMove(format!("folder {} is missing parent id", folder.id))
            })?;

            if !self.folders.contains_key(&parent_id) {
                return Err(FilesystemError::ParentFolderNotFound(parent_id));
            }

            if self.would_create_cycle(folder.id, parent_id) {
                return Err(FilesystemError::InvalidMove(format!(
                    "cycle detected for folder {}",
                    folder.id
                )));
            }

            let normalized = normalize_entry_name(&folder.name)?;
            if normalized != folder.name {
                return Err(FilesystemError::InvalidName(folder.name.clone()));
            }
        }

        for file in self.files.values() {
            if !self.folders.contains_key(&file.parent_id) {
                return Err(FilesystemError::ParentFolderNotFound(file.parent_id));
            }

            let normalized = normalize_entry_name(&file.name)?;
            if normalized != file.name {
                return Err(FilesystemError::InvalidName(file.name.clone()));
            }
        }

        self.ensure_no_name_conflicts()
    }

    fn ensure_no_name_conflicts(&self) -> Result {
        let mut occupied = HashSet::<(Uuid, String)>::new();

        for folder in self.folders.values() {
            if folder.id == ROOT_FOLDER_ID {
                continue;
            }

            let parent_id = folder.parent_id.ok_or_else(|| {
                FilesystemError::InvalidMove(format!("folder {} is missing parent id", folder.id))
            })?;

            let key = (parent_id, folder.name.clone());
            if !occupied.insert(key.clone()) {
                return Err(FilesystemError::NameConflict {
                    parent_id: key.0,
                    name: key.1,
                });
            }
        }

        for file in self.files.values() {
            let key = (file.parent_id, file.name.clone());
            if !occupied.insert(key.clone()) {
                return Err(FilesystemError::NameConflict {
                    parent_id: key.0,
                    name: key.1,
                });
            }
        }

        Ok(())
    }

    fn ensure_name_available(
        &self,
        parent_id: Uuid,
        name: &str,
        ignore_folder_id: Option<Uuid>,
        ignore_file_id: Option<Uuid>,
    ) -> Result {
        let folder_conflict = self.folders.values().any(|folder| {
            folder.id != ROOT_FOLDER_ID
                && folder.parent_id == Some(parent_id)
                && folder.name == name
                && Some(folder.id) != ignore_folder_id
        });
        if folder_conflict {
            return Err(FilesystemError::NameConflict {
                parent_id,
                name: name.to_string(),
            });
        }

        let file_conflict = self.files.values().any(|file| {
            file.parent_id == parent_id && file.name == name && Some(file.id) != ignore_file_id
        });
        if file_conflict {
            return Err(FilesystemError::NameConflict {
                parent_id,
                name: name.to_string(),
            });
        }

        Ok(())
    }

    fn would_create_cycle(&self, folder_id: Uuid, target_parent_id: Uuid) -> bool {
        let mut cursor = Some(target_parent_id);

        while let Some(current) = cursor {
            if current == folder_id {
                return true;
            }

            cursor = self
                .folders
                .get(&current)
                .and_then(|folder| folder.parent_id);
        }

        false
    }
}

fn normalize_entry_name(name: &str) -> Result<String> {
    let trimmed = name.trim();
    if trimmed.is_empty()
        || trimmed == "."
        || trimmed == ".."
        || trimmed.contains('/')
        || trimmed.contains('\0')
    {
        return Err(FilesystemError::InvalidName(name.to_string()));
    }

    Ok(trimmed.to_string())
}
