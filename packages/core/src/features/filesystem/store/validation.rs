use std::collections::HashSet;
use uuid::Uuid;

use super::FilesystemStore;
use super::mutations::normalize_entry_name;
use crate::features::filesystem::error::{FilesystemError, Result};
use crate::features::filesystem::metadata::ROOT_FOLDER_ID;

impl FilesystemStore {
    pub(super) fn validate_snapshot(&self) -> Result {
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

    pub(super) fn ensure_no_name_conflicts(&self) -> Result {
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

    pub(super) fn ensure_name_available(
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

    pub(super) fn would_create_cycle(&self, folder_id: Uuid, target_parent_id: Uuid) -> bool {
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
