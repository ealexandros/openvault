use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use validator::ValidationError;

use crate::features::filesystem::errors::{FilesystemError, Result};
use crate::features::filesystem::models::{
    FILESYSTEM_ROOT_FOLDER_ID, FILESYSTEM_ROOT_FOLDER_NAME, FileMetadata, FolderMetadata,
};

pub fn validate_snapshot(
    folders: &HashMap<Uuid, FolderMetadata>,
    files: &HashMap<Uuid, FileMetadata>,
) -> Result {
    validate_root(folders)?;

    for folder in folders
        .values()
        .filter(|f| f.id != FILESYSTEM_ROOT_FOLDER_ID)
    {
        validate_folder_invariants(folder, folders)?;
    }

    for file in files.values() {
        validate_file_invariants(file, folders)?;
    }

    validate_unique_names(folders, files)?;

    Ok(())
}

fn validate_unique_names(
    folders: &HashMap<Uuid, FolderMetadata>,
    files: &HashMap<Uuid, FileMetadata>,
) -> Result {
    let mut occupied = HashSet::<(Uuid, &str)>::new();

    for folder in folders.values() {
        if folder.id == FILESYSTEM_ROOT_FOLDER_ID {
            continue;
        }

        let parent_id = folder.parent_id.unwrap();

        if !occupied.insert((parent_id, &folder.name)) {
            return Err(FilesystemError::name_conflict(
                parent_id,
                folder.name.as_str(),
            ));
        }
    }

    for file in files.values() {
        if !occupied.insert((file.parent_id, &file.name)) {
            return Err(FilesystemError::name_conflict(
                file.parent_id,
                file.name.as_str(),
            ));
        }
    }

    Ok(())
}

pub fn validate_no_cycle(
    folders: &HashMap<Uuid, FolderMetadata>,
    folder_id: Uuid,
    target_parent_id: Uuid,
) -> Result {
    let mut cursor = Some(target_parent_id);

    while let Some(current) = cursor {
        if current == folder_id {
            return Err(FilesystemError::CycleDetected(folder_id));
        }
        cursor = folders.get(&current).and_then(|f| f.parent_id);
    }

    Ok(())
}

fn validate_root(folders: &HashMap<Uuid, FolderMetadata>) -> Result {
    let root =
        folders
            .get(&FILESYSTEM_ROOT_FOLDER_ID)
            .ok_or(FilesystemError::RootFolderInvariant(
                "root folder is missing".into(),
            ))?;

    if root.parent_id.is_some() {
        return Err(FilesystemError::RootFolderMustNotHaveParent);
    }

    if root.name != FILESYSTEM_ROOT_FOLDER_NAME {
        return Err(FilesystemError::RootFolderMustHaveName);
    }

    Ok(())
}

fn validate_folder_invariants(
    folder: &FolderMetadata,
    folders: &HashMap<Uuid, FolderMetadata>,
) -> Result {
    let parent_id = folder
        .parent_id
        .ok_or(FilesystemError::InvalidMove(format!(
            "folder {} is missing parent id",
            folder.id
        )))?;

    if !folders.contains_key(&parent_id) {
        return Err(FilesystemError::ParentFolderNotFound(parent_id));
    }

    validate_no_cycle(folders, folder.id, parent_id)?;

    Ok(())
}

fn validate_file_invariants(
    file: &FileMetadata,
    folders: &HashMap<Uuid, FolderMetadata>,
) -> Result {
    if !folders.contains_key(&file.parent_id) {
        return Err(FilesystemError::ParentFolderNotFound(file.parent_id));
    }

    Ok(())
}

pub fn validate_file_name(
    parent_id: Uuid,
    name: &str,
    files: &HashMap<Uuid, FileMetadata>,
) -> Result {
    if is_file_name_taken(parent_id, name, files) {
        return Err(FilesystemError::name_conflict(parent_id, name));
    }

    Ok(())
}

pub fn validate_folder_name(
    parent_id: Uuid,
    name: &str,
    folders: &HashMap<Uuid, FolderMetadata>,
) -> Result {
    if is_folder_name_taken(parent_id, name, folders) {
        return Err(FilesystemError::name_conflict(parent_id, name));
    }

    Ok(())
}

pub fn validate_safe_name(name: &str) -> std::result::Result<(), ValidationError> {
    if name == "." || name == ".." {
        return Err(ValidationError::new("reserved_name"));
    }

    if name.trim() != name {
        return Err(ValidationError::new("leading_or_trailing_space"));
    }

    if name.chars().any(|c| c.is_control()) {
        return Err(ValidationError::new("invalid_characters"));
    }

    Ok(())
}

pub fn is_file_name_taken(
    parent_id: Uuid,
    name: &str,
    files: &HashMap<Uuid, FileMetadata>,
) -> bool {
    files
        .values()
        .any(|file| file.parent_id == parent_id && file.name == name)
}

pub fn is_folder_name_taken(
    parent_id: Uuid,
    name: &str,
    folders: &HashMap<Uuid, FolderMetadata>,
) -> bool {
    folders.values().any(|folder| {
        folder.id != FILESYSTEM_ROOT_FOLDER_ID
            && folder.parent_id == Some(parent_id)
            && folder.name == name
    })
}
