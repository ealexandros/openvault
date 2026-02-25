use std::collections::HashMap;
use std::path::{Path, PathBuf};

use uuid::Uuid;
use walkdir::WalkDir;

use super::metadata::{FolderMetadata, ROOT_FOLDER_ID};
use crate::errors::{Error, Result};

pub fn scan_directory(root: &Path) -> Result<(Vec<PathBuf>, Vec<FolderMetadata>)> {
    let mut files = Vec::new();
    let mut folders = vec![FolderMetadata::root()];
    let mut id_map: HashMap<PathBuf, Uuid> = HashMap::new();

    id_map.insert(PathBuf::from(""), ROOT_FOLDER_ID);

    for entry in WalkDir::new(root).sort_by_file_name().min_depth(1) {
        let entry = entry?;
        let absolute = entry.path();

        let relative = absolute
            .strip_prefix(root)
            .map_err(|_| Error::InvalidPath)?;

        let parent = relative.parent().unwrap_or(Path::new(""));
        let parent_id = id_map.get(parent).copied().ok_or(Error::InvalidPath)?;

        if entry.file_type().is_dir() {
            let folder = FolderMetadata::new(Some(parent_id), file_name(relative)?);
            id_map.insert(relative.to_path_buf(), folder.id);
            folders.push(folder);
        } else {
            files.push(relative.to_path_buf());
        }
    }

    Ok((files, folders))
}

fn file_name(path: &Path) -> Result<String> {
    path.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .ok_or(Error::InvalidPath)
}
