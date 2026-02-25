use std::path::Path;

use uuid::Uuid;

use openvault_core::features::filesystem::{
    FileMetadata, FileMetadataPatch, FilesystemStore, FolderMetadata, FolderMetadataPatch,
};
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

    pub fn refresh(&mut self) -> Result<&mut FilesystemStore> {
        *self.store = load_filesystem_store(self.session)?;
        Ok(self.store)
    }

    pub fn commit(&mut self) -> Result<bool> {
        commit_filesystem_store(self.session, self.store).map_err(Into::into)
    }

    pub fn get_file_content(&mut self, id: Uuid) -> Result<Option<Vec<u8>>> {
        let file = self.store.file(&id).cloned();

        if let Some(file) = file {
            let blob = get_blob(&mut self.session, &file.blob)?;
            Ok(Some(blob))
        } else {
            Ok(None)
        }
    }

    pub fn add_folder(&mut self, parent_id: Uuid, name: String) -> Result<Uuid> {
        self.store.add_folder(parent_id, name).map_err(map_fs_error)
    }

    pub fn add_file(&mut self, parent_id: Uuid, source_path: &Path) -> Result<Uuid> {
        add_file(&mut self.session, &mut self.store, parent_id, source_path)
            .map_err(|_| Error::Filesystem("Failed to add file".to_string()))
    }

    pub fn update_folder(&mut self, id: Uuid, patch: FolderMetadataPatch) -> Result {
        self.store.patch_folder(id, patch).map_err(map_fs_error)
    }

    pub fn update_file(&mut self, id: Uuid, patch: FileMetadataPatch) -> Result {
        self.store.patch_file(id, patch).map_err(map_fs_error)
    }

    pub fn delete_folder(&mut self, id: Uuid) -> Result {
        self.store.remove_folder(id).map_err(map_fs_error)
    }

    pub fn delete_file(&mut self, id: Uuid) -> Result {
        self.store.remove_file(id).map_err(map_fs_error)
    }

    pub fn browse(&mut self, parent_id: &Uuid) -> Result<(Vec<FolderMetadata>, Vec<FileMetadata>)> {
        self.store.browse(parent_id).map_err(map_fs_error)
    }

    pub fn file(&mut self, id: &Uuid) -> Result<Option<FileMetadata>> {
        Ok(self.store.file(id).cloned())
    }

    pub fn files(&mut self, parent_id: Uuid) -> Result<Vec<FileMetadata>> {
        Ok(self.store.files(parent_id))
    }

    pub fn folders(&mut self, parent_id: Uuid) -> Result<Vec<FolderMetadata>> {
        Ok(self.store.folders(parent_id))
    }
}

fn map_fs_error(error: openvault_core::features::filesystem::FilesystemError) -> Error {
    Error::from(openvault_core::errors::Error::from(error))
}
