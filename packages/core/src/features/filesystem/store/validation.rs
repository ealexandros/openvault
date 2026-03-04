use std::collections::{HashMap, HashSet};
use std::path::Path;

use uuid::Uuid;

use super::FilesystemStore;
use super::mutations::normalize_entry_name;
use crate::features::filesystem::FolderMetadata;
use crate::features::filesystem::error::{FilesystemError, Result};
use crate::features::filesystem::metadata::FileMetadata;
use crate::features::filesystem::metadata::ROOT_FOLDER_ID;

// @todo-now refactor this..

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ConflictTarget {
    #[default]
    All,
    Folder,
    File,
}

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

    pub(super) fn validate_name_available(
        &self,
        parent_id: Uuid,
        name: &str,
        target: ConflictTarget,
    ) -> Result {
        let folder_taken = matches!(target, ConflictTarget::Folder | ConflictTarget::All)
            && is_folder_name_taken(parent_id, name, &self.folders);

        let file_taken = matches!(target, ConflictTarget::File | ConflictTarget::All)
            && is_file_name_taken(parent_id, name, &self.files);

        if folder_taken || file_taken {
            return Err(FilesystemError::NameConflict {
                parent_id,
                name: name.to_string(),
            });
        }

        Ok(())
    }

    pub(super) fn get_available_file_name(&self, parent_id: Uuid, name: &str) -> Result<String> {
        let normalized_name = normalize_entry_name(name)?;
        self.get_available_name(parent_id, &normalized_name, ConflictTarget::File)
    }

    pub(super) fn get_available_folder_name(&self, parent_id: Uuid, name: &str) -> Result<String> {
        let normalized_name = normalize_entry_name(name)?;
        self.get_available_name(parent_id, &normalized_name, ConflictTarget::Folder)
    }

    fn get_available_name(
        &self,
        parent_id: Uuid,
        name: &str,
        target: ConflictTarget,
    ) -> Result<String> {
        if !self.folders.contains_key(&parent_id) {
            return Err(FilesystemError::ParentFolderNotFound(parent_id));
        }

        let normalized_name = normalize_entry_name(name)?;

        if self
            .validate_name_available(parent_id, &normalized_name, target.clone())
            .is_ok()
        {
            return Ok(normalized_name);
        }

        let generate_candidate = |i: u32| match target {
            ConflictTarget::File => add_suffix_to_file(&normalized_name, i),
            ConflictTarget::Folder | ConflictTarget::All => format!("{normalized_name} ({i})"),
        };

        for i in 1.. {
            let candidate = generate_candidate(i);

            if self
                .validate_name_available(parent_id, &candidate, target.clone())
                .is_ok()
            {
                return Ok(candidate);
            }

            if i == u32::MAX {
                return Err(FilesystemError::NameExhausted {
                    parent_id,
                    name: normalized_name.clone(),
                });
            }
        }

        unreachable!()
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

fn is_file_name_taken(parent_id: Uuid, name: &str, files: &HashMap<Uuid, FileMetadata>) -> bool {
    files
        .values()
        .any(|file| file.parent_id == parent_id && file.name == name)
}

fn is_folder_name_taken(
    parent_id: Uuid,
    name: &str,
    folders: &HashMap<Uuid, FolderMetadata>,
) -> bool {
    folders.values().any(|folder| {
        folder.id != ROOT_FOLDER_ID && folder.parent_id == Some(parent_id) && folder.name == name
    })
}

fn add_suffix_to_file(file_name: &str, n: u32) -> String {
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
