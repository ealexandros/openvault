use std::collections::HashMap;
use std::path::{Path, PathBuf};

use uuid::Uuid;
use walkdir::WalkDir;

use crate::errors::{Error, Result};

use super::metadata::{FileMetadata, FolderMetadata, ROOT_FOLDER_ID};

pub fn scan_directory(root: &Path) -> Result<(Vec<FileMetadata>, Vec<FolderMetadata>)> {
    let mut files = Vec::new();
    let mut folders = vec![FolderMetadata::root()];
    let mut id_map: HashMap<PathBuf, Uuid> = HashMap::new();
    id_map.insert(PathBuf::new(), ROOT_FOLDER_ID);

    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();

        if path == root {
            continue;
        }

        let relative = path.strip_prefix(root).map_err(|_| Error::InvalidPath)?;
        let parent_path = relative.parent().unwrap_or(Path::new(""));

        let parent_id = id_map.get(parent_path).copied().ok_or(Error::InvalidPath)?;

        let name = file_name(relative)?;

        if entry.file_type().is_dir() {
            let folder = FolderMetadata::new(Some(parent_id), name);
            let id = folder.id;
            folders.push(folder);
            id_map.insert(relative.to_path_buf(), id);
            continue;
        }

        files.push(FileMetadata::new(parent_id, name));
    }

    Ok((files, folders))
}

pub fn scan_file(path: &Path) -> Result<FileMetadata> {
    let name = file_name(path)?;
    Ok(FileMetadata::new(ROOT_FOLDER_ID, name))
}

fn file_name(path: &Path) -> Result<String> {
    path.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .ok_or(Error::InvalidPath)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    use uuid::Uuid;

    use super::{scan_directory, scan_file};
    use crate::features::filesystem::metadata::ROOT_FOLDER_ID;
    use crate::features::filesystem::records::FilesystemSnapshot;
    use crate::features::filesystem::store::FilesystemStore;

    fn make_temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("openvault_scanner_{name}_{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).expect("create temp directory");
        dir
    }

    #[test]
    fn directory_scan_output_restores_into_store() {
        let root = make_temp_dir("restore");
        let nested_dir = root.join("docs");
        fs::create_dir_all(&nested_dir).expect("create nested dir");
        fs::write(root.join("readme.md"), b"root").expect("write root file");
        fs::write(nested_dir.join("notes.txt"), b"nested").expect("write nested file");

        let (files, folders) = scan_directory(&root).expect("scan directory");

        let root_folder = folders
            .iter()
            .find(|f| f.id == ROOT_FOLDER_ID)
            .expect("root folder");
        assert_eq!(root_folder.parent_id, None);
        assert_eq!(root_folder.name, "/");

        assert!(
            folders
                .iter()
                .filter(|f| f.id != ROOT_FOLDER_ID)
                .all(|f| !f.name.contains('/'))
        );

        let snapshot = FilesystemSnapshot::new(
            folders
                .into_iter()
                .map(|folder| (folder.id, folder))
                .collect::<HashMap<_, _>>(),
            files
                .into_iter()
                .map(|file| (file.id, file))
                .collect::<HashMap<_, _>>(),
        );

        FilesystemStore::restore(snapshot, vec![]).expect("restore scanned snapshot");
        fs::remove_dir_all(&root).expect("cleanup temp directory");
    }

    #[test]
    fn scan_file_assigns_root_parent() {
        let file = scan_file(Path::new("document.txt")).expect("scan file");
        assert_eq!(file.parent_id, ROOT_FOLDER_ID);
        assert_eq!(file.name, "document.txt");
    }
}
