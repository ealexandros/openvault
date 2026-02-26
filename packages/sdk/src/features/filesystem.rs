use std::path::Path;

use uuid::Uuid;

use openvault_core::features::filesystem::{FileMetadata, FilesystemStore, FolderMetadata};
use openvault_core::operations::blob::get_blob;
use openvault_core::operations::filesystem::{
    add_file, commit_filesystem_store, load_filesystem_store,
};
use openvault_core::vault::runtime::VaultSession;

use crate::errors::{Error, Result};

pub struct FilesystemFeature<'a> {
    session: &'a mut VaultSession,
    store: &'a mut FilesystemStore,
}

impl<'a> FilesystemFeature<'a> {
    pub(crate) fn new(session: &'a mut VaultSession, store: &'a mut FilesystemStore) -> Self {
        Self { session, store }
    }

    pub fn read_file_content(&mut self, id: Uuid) -> Result<Option<Vec<u8>>> {
        let Some(file) = self.store.file(&id) else {
            return Ok(None);
        };

        Ok(Some(get_blob(&mut self.session, &file.blob)?))
    }

    pub fn browse(&mut self, parent_id: &Uuid) -> Result<(Vec<FolderMetadata>, Vec<FileMetadata>)> {
        self.store.browse(parent_id).map_err(map_fs_error)
    }

    pub fn add_folder(&mut self, parent_id: Uuid, name: String) -> Result<Uuid> {
        self.store.add_folder(parent_id, name).map_err(map_fs_error)
    }

    pub fn add_file(&mut self, parent_id: Uuid, source_path: &Path) -> Result<Uuid> {
        add_file(&mut self.session, &mut self.store, parent_id, source_path)
            .map_err(|_| Error::Filesystem("Failed to add file".to_string()))
    }

    pub fn rename_folder(&mut self, id: Uuid, new_name: String) -> Result {
        self.store.rename_folder(id, new_name).map_err(map_fs_error)
    }

    pub fn rename_file(&mut self, id: Uuid, new_name: String) -> Result {
        self.store.rename_file(id, new_name).map_err(map_fs_error)
    }

    pub fn remove_folder(&mut self, id: Uuid) -> Result {
        self.store.remove_folder(id).map_err(map_fs_error)
    }

    pub fn remove_file(&mut self, id: Uuid) -> Result {
        self.store.remove_file(id).map_err(map_fs_error)
    }

    pub fn children_count(&self, parent_id: &Uuid) -> usize {
        self.store.children_count(parent_id)
    }

    pub fn reload(&mut self) -> Result<&FilesystemStore> {
        *self.store = load_filesystem_store(self.session)?;
        Ok(&self.store)
    }

    pub fn commit(&mut self) -> Result<bool> {
        commit_filesystem_store(self.session, self.store).map_err(Into::into)
    }
}

fn map_fs_error(error: openvault_core::features::filesystem::FilesystemError) -> Error {
    Error::from(openvault_core::errors::Error::from(error))
}
