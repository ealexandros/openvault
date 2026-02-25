use std::io::{Cursor, Read};

use openvault_core::features::filesystem::{FilesystemChange, FilesystemStore};
use openvault_core::features::secrets::{SecretStore, SecretsChange};
use openvault_core::features::shared::blob_ref::BlobRef;
use openvault_core::operations::blob::{get_blob, put_blob};
use openvault_core::operations::filesystem::{
    apply_filesystem_change, commit_filesystem_store, load_filesystem_store,
};
use openvault_core::operations::replay::replay_since_checkpoint;
use openvault_core::operations::secrets::{
    apply_secrets_change, commit_secret_store, load_secret_store,
};
use openvault_core::vault::runtime::VaultSession;
use openvault_core::vault::versions::shared::replay::ReplayState;

use crate::error::Result;
use crate::features::{FilesystemFeature, SecretsFeature};
use crate::stores::{StoresCommitResult, VaultStores};

pub struct VaultHandle {
    inner: VaultSession,
    stores: Option<VaultStores>,
}

impl VaultHandle {
    pub(crate) fn new(inner: VaultSession) -> Self {
        Self {
            inner,
            stores: None,
        }
    }

    pub fn version(&self) -> u16 {
        self.inner.version()
    }

    pub fn put_blob(&mut self, source: &mut dyn Read) -> Result<BlobRef> {
        put_blob(&mut self.inner, source).map_err(Into::into)
    }

    pub fn put_blob_bytes(&mut self, bytes: &[u8]) -> Result<BlobRef> {
        let mut cursor = Cursor::new(bytes);
        self.put_blob(&mut cursor)
    }

    pub fn get_blob(&mut self, blob_ref: &BlobRef) -> Result<Vec<u8>> {
        get_blob(&mut self.inner, blob_ref).map_err(Into::into)
    }

    pub fn replay_since_checkpoint(&mut self) -> Result<ReplayState> {
        replay_since_checkpoint(&mut self.inner).map_err(Into::into)
    }

    pub fn filesystem(&mut self) -> FilesystemFeature<'_> {
        FilesystemFeature::new(self)
    }

    pub fn secrets(&mut self) -> SecretsFeature<'_> {
        SecretsFeature::new(self)
    }

    pub fn stores(&self) -> Option<&VaultStores> {
        self.stores.as_ref()
    }

    pub fn stores_mut(&mut self) -> Option<&mut VaultStores> {
        self.stores.as_mut()
    }

    pub fn load_stores(&mut self) -> Result<&mut VaultStores> {
        if self.stores.is_none() {
            self.stores = Some(self.read_all_stores()?);
        }

        Ok(self.stores.as_mut().expect("stores loaded"))
    }

    pub fn refresh_stores(&mut self) -> Result<&mut VaultStores> {
        self.stores = Some(self.read_all_stores()?);
        Ok(self.stores.as_mut().expect("stores refreshed"))
    }

    pub fn clear_stores(&mut self) {
        self.stores = None;
    }

    pub fn filesystem_store(&mut self) -> Result<&mut FilesystemStore> {
        let stores = self.load_stores()?;
        Ok(&mut stores.filesystem)
    }

    pub fn secret_store(&mut self) -> Result<&mut SecretStore> {
        let stores = self.load_stores()?;
        Ok(&mut stores.secrets)
    }

    pub fn commit_stores(&mut self) -> Result<StoresCommitResult> {
        let Some(stores) = self.stores.as_mut() else {
            return Ok(StoresCommitResult::default());
        };

        let filesystem = commit_filesystem_store(&mut self.inner, &mut stores.filesystem)?;
        let secrets = commit_secret_store(&mut self.inner, &mut stores.secrets)?;

        Ok(StoresCommitResult {
            filesystem,
            secrets,
        })
    }

    pub fn commit_all(&mut self) -> Result<StoresCommitResult> {
        self.commit_stores()
    }

    pub fn load_filesystem_store(&mut self) -> Result<FilesystemStore> {
        load_filesystem_store(&mut self.inner).map_err(Into::into)
    }

    pub fn apply_filesystem_change(&mut self, change: FilesystemChange) -> Result<u64> {
        apply_filesystem_change(&mut self.inner, change).map_err(Into::into)
    }

    pub fn commit_filesystem_store(&mut self, store: &mut FilesystemStore) -> Result<bool> {
        commit_filesystem_store(&mut self.inner, store).map_err(Into::into)
    }

    pub fn load_secret_store(&mut self) -> Result<SecretStore> {
        load_secret_store(&mut self.inner).map_err(Into::into)
    }

    pub fn apply_secrets_change(&mut self, change: SecretsChange) -> Result<u64> {
        apply_secrets_change(&mut self.inner, change).map_err(Into::into)
    }

    pub fn commit_secret_store(&mut self, store: &mut SecretStore) -> Result<bool> {
        commit_secret_store(&mut self.inner, store).map_err(Into::into)
    }

    pub fn as_inner(&self) -> &VaultSession {
        &self.inner
    }

    pub fn as_inner_mut(&mut self) -> &mut VaultSession {
        &mut self.inner
    }

    pub fn into_inner(self) -> VaultSession {
        self.inner
    }

    fn read_all_stores(&mut self) -> Result<VaultStores> {
        let filesystem = load_filesystem_store(&mut self.inner)?;
        let secrets = load_secret_store(&mut self.inner)?;
        Ok(VaultStores::new(filesystem, secrets))
    }

    pub(crate) fn commit_filesystem_cached(&mut self) -> Result<bool> {
        let Some(stores) = self.stores.as_mut() else {
            return Ok(false);
        };

        commit_filesystem_store(&mut self.inner, &mut stores.filesystem).map_err(Into::into)
    }

    pub(crate) fn commit_secret_cached(&mut self) -> Result<bool> {
        let Some(stores) = self.stores.as_mut() else {
            return Ok(false);
        };

        commit_secret_store(&mut self.inner, &mut stores.secrets).map_err(Into::into)
    }
}
