use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::FilesystemStore;
use super::mutations::normalize_entry_name;
use crate::features::filesystem::FolderMetadata;
use crate::features::filesystem::error::{FilesystemError, Result};
use crate::features::filesystem::metadata::FileMetadata;
use crate::features::filesystem::metadata::ROOT_FOLDER_ID;

impl FilesystemStore {
    pub(super) fn validate_snapshot(&self) -> Result {
        validate_root(&self.folders)?;

        for folder in self.non_root_folders() {
            validate_folder(folder, &self.folders)?;
        }

        for file in self.files.values() {
            validate_file(file, &self.folders)?;
        }

        ensure_no_name_conflicts(&self.folders, &self.files)?;

        Ok(())
    }

    pub(super) fn ensure_name_available(
        &self,
        parent_id: Uuid,
        name: &str,
        ignore_folder_id: Option<Uuid>,
        ignore_file_id: Option<Uuid>,
    ) -> Result {
        let is_name_not_available = name_conflict(
            parent_id,
            name,
            &self.folders,
            &self.files,
            ignore_folder_id,
            ignore_file_id,
        );

        if is_name_not_available {
            return Err(FilesystemError::NameConflict {
                parent_id,
                name: name.to_string(),
            });
        }

        Ok(())
    }

    pub(super) fn would_create_cycle(&self, folder_id: Uuid, target_parent_id: Uuid) -> bool {
        would_create_cycle(&self.folders, folder_id, target_parent_id)
    }

    fn non_root_folders(&self) -> impl Iterator<Item = &FolderMetadata> {
        self.folders.values().filter(|f| f.id != ROOT_FOLDER_ID)
    }
}

fn validate_root(folders: &HashMap<Uuid, FolderMetadata>) -> Result {
    let root = folders
        .get(&ROOT_FOLDER_ID)
        .ok_or_else(|| FilesystemError::RootFolderInvariant("root folder is missing".into()))?;

    if root.parent_id.is_some() {
        return Err(FilesystemError::RootFolderInvariant(
            "root folder must not have a parent".into(),
        ));
    }

    if root.name != "/" {
        return Err(FilesystemError::RootFolderInvariant(
            "root folder name must be '/'".into(),
        ));
    }

    Ok(())
}

fn validate_folder(folder: &FolderMetadata, folders: &HashMap<Uuid, FolderMetadata>) -> Result {
    let parent_id = folder.parent_id.ok_or_else(|| {
        FilesystemError::InvalidMove(format!("folder {} is missing parent id", folder.id))
    })?;

    if !folders.contains_key(&parent_id) {
        return Err(FilesystemError::ParentFolderNotFound(parent_id));
    }

    if would_create_cycle(folders, folder.id, parent_id) {
        return Err(FilesystemError::InvalidMove(format!(
            "cycle detected for folder {}",
            folder.id
        )));
    }

    validate_name(&folder.name)
}

fn validate_file(file: &FileMetadata, folders: &HashMap<Uuid, FolderMetadata>) -> Result {
    if !folders.contains_key(&file.parent_id) {
        return Err(FilesystemError::ParentFolderNotFound(file.parent_id));
    }

    validate_name(&file.name)
}

fn validate_name(name: &str) -> Result {
    let normalized = normalize_entry_name(name)?;

    if normalized != name {
        return Err(FilesystemError::InvalidName(name.to_string()));
    }

    Ok(())
}

fn ensure_no_name_conflicts(
    folders: &HashMap<Uuid, FolderMetadata>,
    files: &HashMap<Uuid, FileMetadata>,
) -> Result {
    let mut occupied = HashSet::<(Uuid, &str)>::new();

    for folder in folders.values() {
        if folder.id == ROOT_FOLDER_ID {
            continue;
        }

        let parent_id = folder.parent_id.unwrap();

        if !occupied.insert((parent_id, &folder.name)) {
            return Err(FilesystemError::NameConflict {
                parent_id,
                name: folder.name.clone(),
            });
        }
    }

    for file in files.values() {
        if !occupied.insert((file.parent_id, &file.name)) {
            return Err(FilesystemError::NameConflict {
                parent_id: file.parent_id,
                name: file.name.clone(),
            });
        }
    }

    Ok(())
}

fn would_create_cycle(
    folders: &HashMap<Uuid, FolderMetadata>,
    folder_id: Uuid,
    target_parent_id: Uuid,
) -> bool {
    let mut cursor = Some(target_parent_id);

    while let Some(current) = cursor {
        if current == folder_id {
            return true;
        }

        cursor = folders.get(&current).and_then(|f| f.parent_id);
    }

    false
}

fn name_conflict(
    parent_id: Uuid,
    name: &str,
    folders: &HashMap<Uuid, FolderMetadata>,
    files: &HashMap<Uuid, FileMetadata>,
    ignore_folder_id: Option<Uuid>,
    ignore_file_id: Option<Uuid>,
) -> bool {
    folders.values().any(|folder| {
        folder.id != ROOT_FOLDER_ID
            && folder.parent_id == Some(parent_id)
            && folder.name == name
            && Some(folder.id) != ignore_folder_id
    }) || files.values().any(|file| {
        file.parent_id == parent_id && file.name == name && Some(file.id) != ignore_file_id
    })
}
