use crate::crypto::hashing;
use crate::errors::{Error, Result};
use crate::vault::v1::schema::entries::{FileMeta, FolderMeta};
use std::path::Path;
use walkdir::WalkDir;

fn to_vault_path(path: &Path) -> String {
    format!("/{}", path.to_string_lossy())
}

pub fn scan_filesystem(root: &Path) -> Result<(Vec<FileMeta>, Vec<FolderMeta>)> {
    let mut files = Vec::new();
    let mut folders = vec![FolderMeta::root()];

    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();

        if path == root {
            continue;
        }

        let relative = path.strip_prefix(root).map_err(|_| Error::InvalidPath)?;
        let vault_path = to_vault_path(relative);
        let parent_id = relative.parent().map(|p| hashing::crc32(&to_vault_path(p)));

        let id = hashing::crc32(&vault_path);

        if entry.file_type().is_dir() {
            folders.push(FolderMeta::new(id, parent_id, vault_path));
            continue;
        }

        let name = relative
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or(Error::InvalidPath)?;

        let metadata = entry.metadata()?;
        let relative_path = relative.to_path_buf();

        files.push(FileMeta::new(
            id,
            parent_id,
            name,
            metadata.len(),
            relative_path,
        ));
    }

    Ok((files, folders))
}
