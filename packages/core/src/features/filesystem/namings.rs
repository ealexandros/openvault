use std::path::Path;

use super::{FilesystemError, validate};

use crate::features::filesystem::{FileMetadata, FolderMetadata, Result};
use std::collections::HashMap;
use uuid::Uuid;

const MAX_NAME_ATTEMPTS: u32 = 1000;

pub fn generate_folder_name(
    folders: &HashMap<Uuid, FolderMetadata>,
    parent_id: Uuid,
    name: &str,
) -> Result<String> {
    if !folders.contains_key(&parent_id) {
        return Err(FilesystemError::ParentFolderNotFound(parent_id));
    }

    if !validate::is_folder_name_taken(parent_id, name, folders) {
        return Ok(name.to_string());
    }

    for i in 1..MAX_NAME_ATTEMPTS {
        let candidate = format!("{name} ({i})");

        if !validate::is_folder_name_taken(parent_id, &candidate, folders) {
            return Ok(candidate);
        }
    }

    Err(FilesystemError::name_exhausted(parent_id, name))
}

pub fn generate_file_name(
    files: &HashMap<Uuid, FileMetadata>,
    folders: &HashMap<Uuid, FolderMetadata>,
    parent_id: Uuid,
    name: &str,
) -> Result<String> {
    if !folders.contains_key(&parent_id) {
        return Err(FilesystemError::ParentFolderNotFound(parent_id));
    }

    if !validate::is_file_name_taken(parent_id, name, files) {
        return Ok(name.to_string());
    }

    for i in 1..MAX_NAME_ATTEMPTS {
        let candidate = add_suffix_to_path(name, i);

        if !validate::is_file_name_taken(parent_id, &candidate, files) {
            return Ok(candidate);
        }
    }

    Err(FilesystemError::name_exhausted(parent_id, name))
}

fn add_suffix_to_path(file_name: &str, n: u32) -> String {
    let path = Path::new(file_name);

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);

    let extension = path.extension().and_then(|e| e.to_str());

    match extension {
        Some(ext) if !ext.is_empty() => format!("{stem} ({n}).{ext}"),
        _ => format!("{stem} ({n})"),
    }
}
