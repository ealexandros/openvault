use std::collections::HashMap;
use std::path::Path;

use uuid::Uuid;
use walkdir::WalkDir;

use crate::domain::models::filesystem::{File, Folder};
use crate::errors::{Error, Result};

fn to_vault_path(path: &Path) -> String {
    format!("/{}", path.to_string_lossy())
}

pub fn scan_filesystem(root: &Path) -> Result<(Vec<File>, Vec<Folder>)> {
    let mut files = Vec::new();
    let mut folders = Vec::new();

    let mut id_map: HashMap<String, Uuid> = HashMap::new();

    let root_id = Uuid::new_v4();

    folders.push(Folder::new(root_id, None, "/".to_string()));
    id_map.insert("/".to_string(), root_id);

    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();

        if path == root {
            continue;
        }

        let relative = path.strip_prefix(root).map_err(|_| Error::InvalidPath)?;

        let vault_path = to_vault_path(relative);

        let parent_path = relative
            .parent()
            .map(to_vault_path)
            .unwrap_or_else(|| "/".to_string());

        let parent_id = id_map
            .get(&parent_path)
            .copied()
            .ok_or(Error::InvalidPath)?;

        let id = Uuid::new_v4();

        if entry.file_type().is_dir() {
            folders.push(Folder::new(id, Some(parent_id), vault_path.clone()));
            id_map.insert(vault_path, id);
            continue;
        }

        let name = relative
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or(Error::InvalidPath)?;

        let metadata = entry.metadata()?;

        files.push(File::new(id, Some(parent_id), name, metadata.len()));
    }

    Ok((files, folders))
}
