use std::collections::HashMap;
use std::path::{Path, PathBuf};

use uuid::Uuid;
use walkdir::WalkDir;

use super::metadata::{FolderMetadata, ROOT_FOLDER_ID};
use crate::errors::{Error, Result};

pub fn scan_directory(
    root: &Path,
    parent_id: Option<Uuid>,
) -> Result<(Vec<FolderMetadata>, HashMap<Uuid, Vec<PathBuf>>)> {
    let mut folders = Vec::new();
    let mut id_map: HashMap<PathBuf, Uuid> = HashMap::new();
    let mut files_by_folder: HashMap<Uuid, Vec<PathBuf>> = HashMap::new();

    let root_id = parent_id.unwrap_or(ROOT_FOLDER_ID);

    id_map.insert(PathBuf::from(""), root_id);

    for entry in WalkDir::new(root).sort_by_file_name().min_depth(1) {
        let entry = entry?;
        let absolute = entry.path();

        let relative = absolute
            .strip_prefix(root)
            .map_err(|_| Error::InvalidPath)?;

        let parent_path = relative.parent().unwrap_or(Path::new(""));
        let parent_id = id_map.get(parent_path).copied().ok_or(Error::InvalidPath)?;

        if entry.file_type().is_dir() {
            let folder = FolderMetadata::new(Some(parent_id), file_name(relative)?);

            id_map.insert(relative.to_path_buf(), folder.id);
            folders.push(folder);
        } else {
            files_by_folder
                .entry(parent_id)
                .or_default()
                .push(entry.path().to_path_buf());
        }
    }

    Ok((folders, files_by_folder))
}

fn file_name(path: &Path) -> Result<String> {
    path.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .ok_or(Error::InvalidPath)
}
