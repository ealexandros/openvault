use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use validator::ValidationError;

use super::error::{Result, SecretError};
use super::models::{LoginEntry, SECRETS_ROOT_FOLDER_ID, SECRETS_ROOT_FOLDER_NAME, SecretFolder};

pub fn validate_snapshot(
    folders: &HashMap<Uuid, SecretFolder>,
    entries: &HashMap<Uuid, LoginEntry>,
) -> Result {
    validate_root(folders)?;

    for folder in folders.values().filter(|f| f.id != SECRETS_ROOT_FOLDER_ID) {
        validate_folder_invariants(folder, folders)?;
    }

    for entry in entries.values() {
        validate_entry_invariants(entry, folders)?;
    }

    validate_unique_names(folders, entries)?;

    Ok(())
}

fn validate_unique_names(
    folders: &HashMap<Uuid, SecretFolder>,
    entries: &HashMap<Uuid, LoginEntry>,
) -> Result {
    let mut occupied = HashSet::<(Uuid, &str)>::new();

    for folder in folders.values() {
        if folder.id == SECRETS_ROOT_FOLDER_ID {
            continue;
        }

        let parent_id = folder.parent_id.unwrap();

        if !occupied.insert((parent_id, &folder.name)) {
            return Err(SecretError::name_conflict(parent_id, folder.name.as_str()));
        }
    }

    for entry in entries.values() {
        if !occupied.insert((entry.folder_id, &entry.name)) {
            return Err(SecretError::name_conflict(
                entry.folder_id,
                entry.name.as_str(),
            ));
        }
    }

    Ok(())
}

pub fn validate_no_cycle(
    folders: &HashMap<Uuid, SecretFolder>,
    folder_id: Uuid,
    target_parent_id: Uuid,
) -> Result {
    let mut cursor = Some(target_parent_id);

    while let Some(current) = cursor {
        if current == folder_id {
            return Ok(());
        }

        cursor = folders.get(&current).and_then(|f| f.parent_id);
    }

    Err(SecretError::CycleDetected(folder_id))
}

fn validate_root(folders: &HashMap<Uuid, SecretFolder>) -> Result {
    let root = folders
        .get(&SECRETS_ROOT_FOLDER_ID)
        .ok_or_else(|| SecretError::RootFolderInvariant("root folder is missing".into()))?;

    if root.parent_id.is_some() {
        return Err(SecretError::RootFolderMustNotHaveParent);
    }

    if root.name != SECRETS_ROOT_FOLDER_NAME {
        return Err(SecretError::RootFolderMustHaveName);
    }

    Ok(())
}

fn validate_folder_invariants(
    folder: &SecretFolder,
    folders: &HashMap<Uuid, SecretFolder>,
) -> Result {
    let parent_id = folder
        .parent_id
        .ok_or_else(|| SecretError::FolderMustHaveParent(folder.id))?;

    if !folders.contains_key(&parent_id) {
        return Err(SecretError::ParentFolderNotFound(parent_id));
    }

    validate_no_cycle(folders, folder.id, parent_id)?;

    Ok(())
}

fn validate_entry_invariants(entry: &LoginEntry, folders: &HashMap<Uuid, SecretFolder>) -> Result {
    if !folders.contains_key(&entry.folder_id) {
        return Err(SecretError::ParentFolderNotFound(entry.folder_id));
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
