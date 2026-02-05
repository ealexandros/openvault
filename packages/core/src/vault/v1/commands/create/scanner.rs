use crate::constants::{DEFAULT_COMPRESSION, NONCE_LEN};
use crate::errors::Result;
use crate::utils::fs::PathExt;
use crate::vault::v1::structure::{FileEntry, FolderEntry};
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_filesystem(root: &Path) -> Result<(Vec<FileEntry>, Vec<FolderEntry>)> {
    let mut files = Vec::new();
    let mut folders = vec![FolderEntry::root()];

    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();

        if path == root {
            continue;
        }

        let relative = path.relative_to(root)?;

        if entry.file_type().is_dir() {
            folders.push(FolderEntry::new(relative));
            continue;
        }

        if entry.file_type().is_file() {
            let metadata = entry.metadata()?;

            files.push(FileEntry {
                path: relative,
                offset: 0,
                compressed_size: 0,
                original_size: metadata.len(),
                nonce: [0u8; NONCE_LEN],
                compression: DEFAULT_COMPRESSION.to_string(),
            });
        }
    }

    Ok((files, folders))
}
