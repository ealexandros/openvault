use uuid::Uuid;

use super::FilesystemStore;
use crate::features::filesystem::error::{FilesystemError, Result};
use crate::features::filesystem::metadata::{
    FileMetadata, FileMetadataPatch, FolderMetadata, FolderMetadataPatch, ROOT_FOLDER_ID,
};
use crate::features::filesystem::records::FilesystemDelta;

impl FilesystemStore {
    pub(super) fn apply_delta_without_tracking(&mut self, delta: &FilesystemDelta) -> Result {
        self.apply_delta(delta, false)
    }

    pub(super) fn apply_delta_with_tracking(&mut self, delta: &FilesystemDelta) -> Result {
        self.apply_delta(delta, true)
    }

    pub(super) fn apply_delta(&mut self, delta: &FilesystemDelta, track_delta: bool) -> Result {
        match delta {
            FilesystemDelta::FolderAdded(folder) => self.internal_insert_folder(folder.clone())?,
            FilesystemDelta::FolderUpdated { id, patch } => {
                self.internal_patch_folder(*id, patch.clone())?
            }
            FilesystemDelta::FolderDeleted { id } => self.internal_remove_folder(*id)?,
            FilesystemDelta::FileAdded(file) => self.internal_insert_file(file.clone())?,
            FilesystemDelta::FileUpdated { id, patch } => {
                self.internal_patch_file(*id, patch.clone())?
            }
            FilesystemDelta::FileDeleted { id } => self.internal_remove_file(*id)?,
        }

        if track_delta {
            self.deltas.push(delta.clone());
        }

        Ok(())
    }

    pub(super) fn internal_insert_folder(&mut self, folder: FolderMetadata) -> Result {
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

    pub(super) fn internal_patch_folder(&mut self, id: Uuid, patch: FolderMetadataPatch) -> Result {
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

    pub(super) fn internal_remove_folder(&mut self, id: Uuid) -> Result {
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

    pub(super) fn internal_insert_file(&mut self, file: FileMetadata) -> Result {
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

    pub(super) fn internal_patch_file(&mut self, id: Uuid, patch: FileMetadataPatch) -> Result {
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

    pub(super) fn internal_remove_file(&mut self, id: Uuid) -> Result {
        if self.files.remove(&id).is_none() {
            return Err(FilesystemError::FileNotFound(id));
        }
        Ok(())
    }
}

pub(super) fn normalize_entry_name(name: &str) -> Result<String> {
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
