use uuid::Uuid;

use openvault_core::features::filesystem::{
    FileMetadata, FileMetadataPatch, FilesystemChange, FilesystemStore, FolderMetadata,
    FolderMetadataPatch,
};
use openvault_core::features::secrets::{
    EncryptedField, LoginEntry, LoginEntryPatch, SecretStore, SecretsChange,
};

use crate::error::{Error, Result};
use crate::session::VaultHandle;

pub trait FeatureFacade {
    type Store;
    type Change;

    fn load(&mut self) -> Result<&mut Self::Store>;
    fn refresh(&mut self) -> Result<&mut Self::Store>;
    fn clear_cache(&mut self);
    fn apply_changes(&mut self, change: Self::Change) -> Result<u64>;
    fn commit(&mut self) -> Result<bool>;
}

pub struct FilesystemFeature<'a> {
    handle: &'a mut VaultHandle,
}

impl<'a> FilesystemFeature<'a> {
    pub(crate) fn new(handle: &'a mut VaultHandle) -> Self {
        Self { handle }
    }

    pub fn add_file(&mut self, file: FileMetadata) -> Result<Uuid> {
        let store = self.load()?;
        store
            .add_file(file)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn add_directory(&mut self, folder: FolderMetadata) -> Result<Uuid> {
        let store = self.load()?;
        store
            .add_folder(folder)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn update_file(&mut self, id: Uuid, patch: FileMetadataPatch) -> Result {
        let store = self.load()?;
        store
            .update_file(id, patch)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn update_directory(&mut self, id: Uuid, patch: FolderMetadataPatch) -> Result {
        let store = self.load()?;
        store
            .update_folder(id, patch)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn delete_file(&mut self, id: Uuid) -> Result {
        let store = self.load()?;
        store
            .delete_file(id)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn delete_directory(&mut self, id: Uuid) -> Result {
        let store = self.load()?;
        store
            .delete_folder(id)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn files(&mut self) -> Result<Vec<FileMetadata>> {
        Ok(self.load()?.files())
    }

    pub fn directories(&mut self) -> Result<Vec<FolderMetadata>> {
        Ok(self.load()?.folders())
    }
}

impl FeatureFacade for FilesystemFeature<'_> {
    type Store = FilesystemStore;
    type Change = FilesystemChange;

    fn load(&mut self) -> Result<&mut Self::Store> {
        self.handle.filesystem_store()
    }

    fn refresh(&mut self) -> Result<&mut Self::Store> {
        self.handle.refresh_stores()?;
        self.handle.filesystem_store()
    }

    fn clear_cache(&mut self) {
        self.handle.clear_stores();
    }

    fn apply_changes(&mut self, change: Self::Change) -> Result<u64> {
        let offset = self.handle.apply_filesystem_change(change)?;
        self.clear_cache();
        Ok(offset)
    }

    fn commit(&mut self) -> Result<bool> {
        self.handle.commit_filesystem_cached()
    }
}

pub struct SecretsFeature<'a> {
    handle: &'a mut VaultHandle,
}

impl<'a> SecretsFeature<'a> {
    pub(crate) fn new(handle: &'a mut VaultHandle) -> Self {
        Self { handle }
    }

    pub fn insert(&mut self, entry: LoginEntry) -> Result<Uuid> {
        let store = self.load()?;
        store
            .insert(entry)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn update(&mut self, id: Uuid, patch: LoginEntryPatch) -> Result {
        let store = self.load()?;
        store
            .update(id, patch)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn delete(&mut self, id: Uuid) -> Result {
        let store = self.load()?;
        store
            .delete(id)
            .map_err(|e| Error::from(openvault_core::errors::Error::from(e)))
    }

    pub fn list_all(&mut self) -> Result<Vec<LoginEntry>> {
        Ok(self.load()?.list_all())
    }

    pub fn list_by_folder(&mut self, folder: &str) -> Result<Vec<LoginEntry>> {
        Ok(self.load()?.list_by_folder(folder))
    }

    pub fn show_password(&mut self, id: &Uuid) -> Result<Option<EncryptedField>> {
        Ok(self.load()?.show_password(id))
    }
}

impl FeatureFacade for SecretsFeature<'_> {
    type Store = SecretStore;
    type Change = SecretsChange;

    fn load(&mut self) -> Result<&mut Self::Store> {
        self.handle.secret_store()
    }

    fn refresh(&mut self) -> Result<&mut Self::Store> {
        self.handle.refresh_stores()?;
        self.handle.secret_store()
    }

    fn clear_cache(&mut self) {
        self.handle.clear_stores();
    }

    fn apply_changes(&mut self, change: Self::Change) -> Result<u64> {
        let offset = self.handle.apply_secrets_change(change)?;
        self.clear_cache();
        Ok(offset)
    }

    fn commit(&mut self) -> Result<bool> {
        self.handle.commit_secret_cached()
    }
}
