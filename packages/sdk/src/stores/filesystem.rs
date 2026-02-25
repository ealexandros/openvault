use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

use uuid::Uuid;

use openvault_core::features::filesystem::{
    FileMetadata, FileMetadataPatch, FilesystemDelta, FilesystemStore as CoreFilesystemStore,
    FolderMetadata, FolderMetadataPatch, scan_file,
};
use openvault_core::features::shared::blob_ref::BlobRef;
use openvault_core::operations::blob::{get_blob, put_blob};
use openvault_core::operations::filesystem::{commit_filesystem_store, load_filesystem_store};
use openvault_core::vault::runtime::VaultSession;

use crate::errors::{Error, Result};

pub struct FilesystemStore<'a> {
    session: &'a mut VaultSession,
    state: Option<CoreFilesystemStore>,
}

impl<'a> FilesystemStore<'a> {
    pub(crate) fn new(session: &'a mut VaultSession) -> Self {
        Self {
            session,
            state: None,
        }
    }

    pub fn load(&mut self) -> Result<&mut CoreFilesystemStore> {
        if self.state.is_none() {
            self.state = Some(load_filesystem_store(self.session)?);
        }

        Ok(self.state.as_mut().expect("state loaded"))
    }

    pub fn refresh(&mut self) -> Result<&mut CoreFilesystemStore> {
        self.state = Some(load_filesystem_store(self.session)?);
        Ok(self.state.as_mut().expect("state refreshed"))
    }

    pub fn clear(&mut self) {
        self.state = None;
    }

    pub fn commit(&mut self) -> Result<bool> {
        let Some(state) = self.state.as_mut() else {
            return Ok(false);
        };
        commit_filesystem_store(self.session, state).map_err(Into::into)
    }

    pub fn put_blob(&mut self, source: &mut dyn Read) -> Result<BlobRef> {
        put_blob(self.session, source).map_err(Into::into)
    }

    pub fn put_blob_bytes(&mut self, bytes: &[u8]) -> Result<BlobRef> {
        let mut cursor = Cursor::new(bytes);
        self.put_blob(&mut cursor)
    }

    pub fn put_blob_path(&mut self, path: &Path) -> Result<BlobRef> {
        let mut file = File::open(path)
            .map_err(openvault_core::errors::Error::from)
            .map_err(Error::from)?;

        self.put_blob(&mut file)
    }

    pub fn get_blob(&mut self, blob_ref: &BlobRef) -> Result<Vec<u8>> {
        get_blob(self.session, blob_ref).map_err(Into::into)
    }

    pub fn add_file(&mut self, file: FileMetadata) -> Result<Uuid> {
        let id = file.id;
        self.add_delta(FilesystemDelta::FileAdded(file))?;
        Ok(id)
    }

    pub fn add_file_with_blob(
        &mut self,
        mut file: FileMetadata,
        blob_ref: BlobRef,
    ) -> Result<Uuid> {
        file.blob = Some(blob_ref);
        self.add_file(file)
    }

    pub fn add_file_from_path(&mut self, path: &Path) -> Result<Uuid> {
        let mut file = scan_file(path).map_err(Error::from)?;
        let blob_ref = self.put_blob_path(path)?;
        file.blob = Some(blob_ref);
        self.add_file(file)
    }

    pub fn update_file(&mut self, id: Uuid, patch: FileMetadataPatch) -> Result {
        self.add_delta(FilesystemDelta::FileUpdated { id, patch })
    }

    pub fn delete_file(&mut self, id: Uuid) -> Result {
        self.add_delta(FilesystemDelta::FileDeleted { id })
    }

    pub fn file(&mut self, id: &Uuid) -> Result<Option<FileMetadata>> {
        Ok(self.load()?.file(id).cloned())
    }

    pub fn files(&mut self) -> Result<Vec<FileMetadata>> {
        Ok(self.load()?.files())
    }

    pub fn get_file(&mut self, id: &Uuid) -> Result<Option<Vec<u8>>> {
        let blob_ref = self
            .load()?
            .file(id)
            .and_then(|file| file.blob.as_ref())
            .cloned();

        match blob_ref {
            Some(blob_ref) => self.get_blob(&blob_ref).map(Some),
            None => Ok(None),
        }
    }

    pub fn add_directory(&mut self, folder: FolderMetadata) -> Result<Uuid> {
        let id = folder.id;
        self.add_delta(FilesystemDelta::FolderAdded(folder))?;
        Ok(id)
    }

    pub fn update_directory(&mut self, id: Uuid, patch: FolderMetadataPatch) -> Result {
        self.add_delta(FilesystemDelta::FolderUpdated { id, patch })
    }

    pub fn delete_directory(&mut self, id: Uuid) -> Result {
        self.add_delta(FilesystemDelta::FolderDeleted { id })
    }

    pub fn directories(&mut self) -> Result<Vec<FolderMetadata>> {
        Ok(self.load()?.folders())
    }

    fn add_delta(&mut self, delta: FilesystemDelta) -> Result {
        let store = self.load()?;

        match delta {
            FilesystemDelta::FolderAdded(folder) => {
                store.add_folder(folder).map(|_| ()).map_err(map_fs_error)
            }
            FilesystemDelta::FolderUpdated { id, patch } => {
                store.update_folder(id, patch).map_err(map_fs_error)
            }
            FilesystemDelta::FolderDeleted { id } => store.delete_folder(id).map_err(map_fs_error),
            FilesystemDelta::FileAdded(file) => {
                store.add_file(file).map(|_| ()).map_err(map_fs_error)
            }
            FilesystemDelta::FileUpdated { id, patch } => {
                store.update_file(id, patch).map_err(map_fs_error)
            }
            FilesystemDelta::FileDeleted { id } => store.delete_file(id).map_err(map_fs_error),
        }
    }
}

fn map_fs_error(error: openvault_core::features::filesystem::FilesystemError) -> Error {
    Error::from(openvault_core::errors::Error::from(error))
}
