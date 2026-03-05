use std::collections::HashMap;
use uuid::Uuid;

use super::models::{FileMetadata, FolderMetadata};

#[derive(Clone, Debug, Default)]
pub(crate) struct FilesystemIndex {
    folders_by_parent: HashMap<Uuid, Vec<Uuid>>,
    files_by_parent: HashMap<Uuid, Vec<Uuid>>,
}

impl FilesystemIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(
        folders: &HashMap<Uuid, FolderMetadata>,
        files: &HashMap<Uuid, FileMetadata>,
    ) -> Self {
        let mut index = Self::new();

        for folder in folders.values() {
            if let Some(parent_id) = folder.parent_id {
                index.add_folder(parent_id, folder.id);
            }
        }

        for file in files.values() {
            index.add_file(file.parent_id, file.id);
        }

        index
    }

    pub fn folders(&self, parent_id: &Uuid) -> &[Uuid] {
        self.folders_by_parent
            .get(parent_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn files(&self, parent_id: &Uuid) -> &[Uuid] {
        self.files_by_parent
            .get(parent_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn children_count(&self, parent_id: &Uuid) -> usize {
        self.folders(parent_id).len() + self.files(parent_id).len()
    }

    pub fn add_folder(&mut self, parent_id: Uuid, folder_id: Uuid) {
        self.folders_by_parent
            .entry(parent_id)
            .or_default()
            .push(folder_id);
    }

    pub fn remove_folder(&mut self, parent_id: Uuid, folder_id: Uuid) {
        Self::remove_child(&mut self.folders_by_parent, parent_id, folder_id);
    }

    pub fn add_file(&mut self, parent_id: Uuid, file_id: Uuid) {
        self.files_by_parent
            .entry(parent_id)
            .or_default()
            .push(file_id);
    }

    pub fn remove_file(&mut self, parent_id: Uuid, file_id: Uuid) {
        Self::remove_child(&mut self.files_by_parent, parent_id, file_id);
    }

    pub fn move_folder(&mut self, from_parent_id: Uuid, to_parent_id: Uuid, folder_id: Uuid) {
        Self::remove_child(&mut self.folders_by_parent, from_parent_id, folder_id);
        self.add_folder(to_parent_id, folder_id);
    }

    pub fn move_file(&mut self, from_parent_id: Uuid, to_parent_id: Uuid, file_id: Uuid) {
        Self::remove_child(&mut self.files_by_parent, from_parent_id, file_id);
        self.add_file(to_parent_id, file_id);
    }

    fn remove_child(map: &mut HashMap<Uuid, Vec<Uuid>>, parent_id: Uuid, child_id: Uuid) {
        let Some(children) = map.get_mut(&parent_id) else {
            return;
        };

        children.retain(|&id| id != child_id);

        if children.is_empty() {
            map.remove(&parent_id);
        }
    }
}
