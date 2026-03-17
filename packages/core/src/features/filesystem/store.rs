use std::collections::HashMap;

use uuid::Uuid;
use validator::Validate;
use zeroize::Zeroize;

use super::errors::{FilesystemError, Result};
use super::events::{FilesystemChange, FilesystemDelta, FilesystemSnapshot};
use super::index::FilesystemIndex;
use super::models::{FileMetadata, FolderMetadata};
use super::patch::{FileMetadataPatch, FolderMetadataPatch};
use super::validate;
use crate::features::filesystem::FILESYSTEM_ROOT_FOLDER_ID;
use crate::features::filesystem::namings::{generate_file_name, generate_folder_name};
use crate::features::shared::{BlobRef, DEFAULT_SNAPSHOT_THRESHOLD};

#[derive(Clone, Debug)]
pub struct FilesystemStore {
    pub(crate) folders: HashMap<Uuid, FolderMetadata>,
    pub(crate) files: HashMap<Uuid, FileMetadata>,
    pub(crate) index: FilesystemIndex,
    pub(crate) deltas: Vec<FilesystemDelta>,
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

        validate::validate_snapshot(&store.folders, &store.files)?;

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
        let folder_ids = self.index.folders(&parent_id);

        folder_ids
            .iter()
            .filter_map(|id| self.folders.get(id))
            .cloned()
            .collect()
    }

    pub fn files(&self, parent_id: Uuid) -> Vec<FileMetadata> {
        let file_ids = self.index.files(&parent_id);

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
        let file_ids = self.index.files(parent_id);

        file_ids
            .iter()
            .filter_map(|id| self.files.get(id))
            .any(|f| f.name == name)
    }

    pub fn folder_children_count(&self, folder_id: &Uuid) -> usize {
        self.index.children_count(folder_id)
    }

    pub fn folder_total_size_bytes(&self, folder_id: &Uuid) -> u64 {
        self.index
            .files(folder_id)
            .iter()
            .map(|id| self.files.get(id).map(|f| f.size_bytes()).unwrap_or(0))
            .sum()
    }

    pub fn add_folder(&mut self, parent_id: Uuid, name: String) -> Result<Uuid> {
        let folder_name = generate_folder_name(&self.folders, parent_id, name.as_str())?;

        let folder = FolderMetadata::new(Some(parent_id), folder_name);
        let folder_id = folder.id;

        self.commit_delta(&FilesystemDelta::FolderAdded(folder))?;

        Ok(folder_id)
    }

    pub fn add_file(
        &mut self,
        parent_id: Uuid,
        name: String,
        extension: String,
        blob: BlobRef,
    ) -> Result<Uuid> {
        let file_name = generate_file_name(&self.files, &self.folders, parent_id, name.as_str())?;

        let file = FileMetadata::new(parent_id, file_name, extension, blob);
        let file_id = file.id;

        self.commit_delta(&FilesystemDelta::FileAdded(file))?;

        Ok(file_id)
    }

    pub fn remove_file(&mut self, id: Uuid) -> Result {
        self.commit_delta(&FilesystemDelta::FileDeleted(id))
    }

    pub fn remove_folder(&mut self, id: Uuid) -> Result {
        self.commit_delta(&FilesystemDelta::FolderDeleted(id))
    }

    pub fn rename_file(&mut self, id: Uuid, new_name: String) -> Result {
        let patch = FileMetadataPatch::rename(new_name);
        self.commit_delta(&FilesystemDelta::FileUpdated { id, patch })
    }

    pub fn rename_folder(&mut self, id: Uuid, new_name: String) -> Result {
        let patch = FolderMetadataPatch::rename(new_name);
        self.commit_delta(&FilesystemDelta::FolderUpdated { id, patch })
    }

    pub fn set_folder_icon(&mut self, id: Uuid, new_icon: String) -> Result {
        let patch = FolderMetadataPatch::set_icon(new_icon);
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

    pub fn set_folder_favorite(&mut self, id: Uuid, is_favourite: bool) -> Result {
        let patch = FolderMetadataPatch::set_favourite(is_favourite);
        self.commit_delta(&FilesystemDelta::FolderUpdated { id, patch })
    }

    pub fn set_file_favorite(&mut self, id: Uuid, is_favourite: bool) -> Result {
        let patch = FileMetadataPatch::set_favourite(is_favourite);
        self.commit_delta(&FilesystemDelta::FileUpdated { id, patch })
    }

    pub fn snapshot(&self) -> FilesystemSnapshot {
        FilesystemSnapshot::new(self.folders.clone(), self.files.clone())
    }

    pub fn pending_changes(&self) -> Option<FilesystemChange> {
        if self.deltas.is_empty() {
            return None;
        }

        if self.deltas.len() >= DEFAULT_SNAPSHOT_THRESHOLD {
            return Some(FilesystemChange::Snapshot(self.snapshot()));
        }

        Some(FilesystemChange::Deltas(self.deltas.clone()))
    }

    pub fn clear_deltas(&mut self) {
        self.deltas.clear();
    }

    fn commit_delta(&mut self, delta: &FilesystemDelta) -> Result {
        self.apply_delta(delta, true)
    }

    fn replay_delta(&mut self, delta: &FilesystemDelta) -> Result {
        self.apply_delta(delta, false)
    }

    fn apply_delta(&mut self, delta: &FilesystemDelta, track_delta: bool) -> Result {
        match delta {
            FilesystemDelta::FolderAdded(folder) => self.apply_folder_added(folder.clone()),
            FilesystemDelta::FolderDeleted(id) => self.apply_folder_deleted(*id),
            FilesystemDelta::FolderUpdated { id, patch } => self.apply_folder_updated(*id, patch),
            FilesystemDelta::FileAdded(file) => self.apply_file_added(file.clone()),
            FilesystemDelta::FileDeleted(id) => self.apply_file_deleted(*id),
            FilesystemDelta::FileUpdated { id, patch } => self.apply_file_updated(*id, patch),
        }?;

        if track_delta {
            self.deltas.push(delta.clone());
        }

        Ok(())
    }

    fn apply_folder_added(&mut self, folder: FolderMetadata) -> Result {
        if folder.id == FILESYSTEM_ROOT_FOLDER_ID {
            return Err(FilesystemError::RootFolderReserved);
        }

        if self.folders.contains_key(&folder.id) || self.files.contains_key(&folder.id) {
            return Err(FilesystemError::DuplicateId(folder.id));
        }

        let parent_id = folder
            .parent_id
            .ok_or(FilesystemError::FolderMustHaveParent(folder.id))?;

        if !self.folders.contains_key(&parent_id) {
            return Err(FilesystemError::ParentFolderNotFound(parent_id));
        }

        validate::validate_folder_name(parent_id, &folder.name, &self.folders)?;

        folder.validate()?;

        self.index.add_folder(parent_id, folder.id);
        self.folders.insert(folder.id, folder);

        Ok(())
    }

    fn apply_folder_deleted(&mut self, id: Uuid) -> Result {
        if id == FILESYSTEM_ROOT_FOLDER_ID {
            return Err(FilesystemError::CannotDeleteRootFolder);
        }

        let files = self.index.files(&id).to_vec();

        for file_id in files {
            self.apply_file_deleted(file_id)?;
        }

        let subfolders = self.index.folders(&id).to_vec();

        for subfolder_id in subfolders {
            self.apply_folder_deleted(subfolder_id)?;
        }

        let folder = self
            .folders
            .get(&id)
            .ok_or(FilesystemError::FolderNotFound(id))?;

        let parent_id = folder
            .parent_id
            .ok_or(FilesystemError::FolderMissingParent(id))?;

        self.index.remove_folder(parent_id, id);
        self.folders.remove(&id);

        Ok(())
    }

    fn apply_folder_updated(&mut self, id: Uuid, patch: &FolderMetadataPatch) -> Result {
        if id == FILESYSTEM_ROOT_FOLDER_ID {
            return Err(FilesystemError::RootFolderImmutable);
        }

        let folder = self
            .folders
            .get(&id)
            .ok_or(FilesystemError::FolderNotFound(id))?;

        let current_parent = folder
            .parent_id
            .ok_or(FilesystemError::FolderMustHaveParent(id))?;

        let target_parent = patch.parent_id.unwrap_or(current_parent);

        if !self.folders.contains_key(&target_parent) {
            return Err(FilesystemError::ParentFolderNotFound(target_parent));
        }

        if target_parent == id {
            validate::validate_no_cycle(&self.folders, id, target_parent)?
        }

        let target_name = patch.name.clone().unwrap_or(folder.name.clone());

        if target_parent != current_parent || target_name != folder.name {
            validate::validate_folder_name(target_parent, &target_name, &self.folders)?;
        }

        if current_parent != target_parent {
            self.index.remove_folder(current_parent, id);
            self.index.add_folder(target_parent, id);
        }

        let folder = self.folders.get_mut(&id).unwrap();

        folder.validate()?;

        folder.parent_id = Some(target_parent);
        folder.name = target_name;
        folder.icon = patch.icon.clone().unwrap_or(folder.icon.clone());
        folder.is_favourite = patch.is_favourite.unwrap_or(folder.is_favourite);
        folder.updated_at = patch.updated_at;

        if patch.parent_id.is_some() {
            self.index.move_folder(current_parent, target_parent, id);
        }

        Ok(())
    }

    fn apply_file_added(&mut self, file: FileMetadata) -> Result {
        if self.files.contains_key(&file.id) || self.folders.contains_key(&file.id) {
            return Err(FilesystemError::DuplicateId(file.id));
        }

        if !self.folders.contains_key(&file.parent_id) {
            return Err(FilesystemError::ParentFolderNotFound(file.parent_id));
        }

        validate::validate_file_name(file.parent_id, &file.name, &self.files)?;

        file.validate()?;

        self.index.add_file(file.parent_id, file.id);
        self.files.insert(file.id, file);

        Ok(())
    }

    fn apply_file_deleted(&mut self, id: Uuid) -> Result {
        let file = self
            .files
            .get(&id)
            .ok_or(FilesystemError::FileNotFound(id))?;

        let parent_id = file.parent_id;

        self.index.remove_file(parent_id, id);
        self.files.remove(&id);

        Ok(())
    }

    fn apply_file_updated(&mut self, id: Uuid, patch: &FileMetadataPatch) -> Result {
        let file = self
            .files
            .get(&id)
            .ok_or(FilesystemError::FileNotFound(id))?;

        let current_parent = file.parent_id;
        let target_parent = patch.parent_id.unwrap_or(current_parent);

        if !self.folders.contains_key(&target_parent) {
            return Err(FilesystemError::ParentFolderNotFound(target_parent));
        }

        let target_name = patch.name.clone().unwrap_or(file.name.clone());

        if target_parent != current_parent || target_name != file.name {
            validate::validate_file_name(target_parent, &target_name, &self.files)?;
        }

        if target_parent != current_parent {
            self.index.remove_file(current_parent, id);
            self.index.add_file(target_parent, id);
        }

        let file = self.files.get_mut(&id).unwrap();

        file.validate()?;

        file.name = target_name;
        file.parent_id = target_parent;
        file.is_favourite = patch.is_favourite.unwrap_or(file.is_favourite);
        file.updated_at = patch.updated_at;

        if let Some(extension) = &patch.extension {
            file.extension = extension.clone();
        }

        if let Some(blob) = &patch.blob {
            file.blob = blob.clone();
        }

        if patch.parent_id.is_some() {
            self.index.move_file(current_parent, target_parent, id);
        }

        Ok(())
    }
}

impl Zeroize for FilesystemStore {
    fn zeroize(&mut self) {
        for folder in self.folders.values_mut() {
            folder.zeroize();
        }
        self.folders.clear();

        for file in self.files.values_mut() {
            file.zeroize();
        }
        self.files.clear();

        for delta in &mut self.deltas {
            delta.zeroize();
        }
        self.deltas.clear();

        self.index = FilesystemIndex::new();
    }
}
