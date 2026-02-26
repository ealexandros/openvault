use std::collections::HashMap;
use uuid::Uuid;

use crate::features::filesystem::{FileMetadata, FolderMetadata};

#[derive(Clone, Debug, Default)]
pub(super) struct FilesystemIndex {
    folder_children: HashMap<Uuid, Vec<Uuid>>,
    file_children: HashMap<Uuid, Vec<Uuid>>,
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
                index.insert_folder(parent_id, folder.id);
            }
        }

        for file in files.values() {
            index.insert_file(file.parent_id, file.id);
        }

        index
    }

    pub fn folder_children(&self, parent_id: &Uuid) -> &[Uuid] {
        self.folder_children
            .get(parent_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn file_children(&self, parent_id: &Uuid) -> &[Uuid] {
        self.file_children
            .get(parent_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn children_count(&self, parent_id: &Uuid) -> usize {
        self.folder_children(parent_id).len() + self.file_children(parent_id).len()
    }

    pub fn insert_folder(&mut self, parent_id: Uuid, id: Uuid) {
        self.folder_children.entry(parent_id).or_default().push(id);
    }

    pub fn remove_folder(&mut self, parent_id: Uuid, id: Uuid) {
        if let Some(children) = self.folder_children.get_mut(&parent_id) {
            children.retain(|&x| x != id);
            if children.is_empty() {
                self.folder_children.remove(&parent_id);
            }
        }
    }

    pub fn insert_file(&mut self, parent_id: Uuid, id: Uuid) {
        self.file_children.entry(parent_id).or_default().push(id);
    }

    pub fn remove_file(&mut self, parent_id: Uuid, id: Uuid) {
        if let Some(children) = self.file_children.get_mut(&parent_id) {
            children.retain(|&x| x != id);
            if children.is_empty() {
                self.file_children.remove(&parent_id);
            }
        }
    }
}
