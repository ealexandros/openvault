pub mod codec;
pub mod params;
pub mod views;

use openvault_crypto::keys::MasterKey;
use uuid::Uuid;

use crate::domain::folders::normalize_folder_path;
use crate::domain::records::{SecretsChange, Snapshot};
use crate::domain::store::SecretStore;
use crate::errors::{Result, SecretError};
use crate::manager::params::{AddSecretEntryParams, UpdateSecretEntryParams};
use crate::manager::views::{FolderListing, LoginEntryView, build_folder_listing};
use crate::wire::v1::changes::SecretsChunkV1;
use crate::wire::v1::mapper as wire_mapper;

// @todo-soon consider adding derived key for data fields

#[derive(Debug)]
pub struct SecretManager {
    key: MasterKey,
    store: SecretStore,
}

impl SecretManager {
    pub fn unlock(key: MasterKey, chunks: Vec<Vec<u8>>) -> Result<SecretManager> {
        if chunks.is_empty() {
            return Ok(SecretManager::create(key));
        }

        let store = SecretManager::restore_store(&key, &chunks)?;

        Ok(Self { key, store })
    }

    fn restore_store(key: &MasterKey, chunks: &[Vec<u8>]) -> Result<SecretStore> {
        let mut snapshot = Snapshot::default();
        let mut deltas = Vec::new();

        for chunk in chunks {
            let decrypted = codec::decrypt(chunk, key)?;
            let changes = decode_changes(&decrypted)?;

            for change in changes {
                match change {
                    SecretsChange::Snapshot(s) => {
                        snapshot = s;
                        deltas.clear();
                    }
                    SecretsChange::Deltas(d) => deltas.extend(d),
                }
            }
        }

        SecretStore::restore(snapshot, deltas)
    }
}

impl SecretManager {
    pub fn create(key: MasterKey) -> Self {
        Self {
            key,
            store: SecretStore::new(),
        }
    }

    pub fn list_all(&self) -> Vec<LoginEntryView> {
        self.store
            .list_all()
            .into_iter()
            .map(|entry| LoginEntryView::from(&entry))
            .collect()
    }

    pub fn browse_folder(&self, folder: &str) -> FolderListing {
        let current_folder = normalize_folder_path(folder);
        let entries = self
            .store
            .list_by_folder(&current_folder)
            .into_iter()
            .map(|entry| LoginEntryView::from(&entry))
            .collect();
        let subfolders = self.store.list_subfolders(&current_folder);

        build_folder_listing(&current_folder, entries, subfolders)
    }

    pub fn get_entry(&self, id: &Uuid) -> Option<LoginEntryView> {
        self.store
            .get_entry(id)
            .map(|entry| LoginEntryView::from(&entry))
    }

    pub fn show_password(&self, id: &Uuid) -> Result<String> {
        let encrypted = self
            .store
            .show_password(id)
            .ok_or_else(|| SecretError::NotFound(id.to_string()))?;

        codec::decrypt_password(encrypted.as_bytes(), &self.key)
    }

    pub fn insert(&mut self, params: AddSecretEntryParams) -> Result<Uuid> {
        let entry = params.into_entry(&self.key)?;
        self.store.insert(entry)
    }

    pub fn update(&mut self, id: &Uuid, params: UpdateSecretEntryParams) -> Result {
        let patch = params.into_patch(&self.key)?;
        self.store.update(*id, patch)
    }

    pub fn delete(&mut self, id: &Uuid) -> Result {
        self.store.delete(*id)
    }

    pub fn export(&self) -> Result<Vec<u8>> {
        let chunk = wire_mapper::encode_changes(vec![self.store.create_snapshot()]);
        let serialized = codec::serialize(&chunk)?;
        codec::encrypt(&serialized, &self.key)
    }

    pub fn export_changes(&self) -> Result<Vec<u8>> {
        let changes: Vec<SecretsChange> = self.store.pending_changes().into_iter().collect();
        let chunk = wire_mapper::encode_changes(changes);
        let serialized = codec::serialize(&chunk)?;
        codec::encrypt(&serialized, &self.key)
    }

    pub fn reset_sync_state(&mut self) {
        self.store.reset_sync_state();
    }

    pub fn lock(self) {
        drop(self);
    }
}

fn decode_changes(payload: &[u8]) -> Result<Vec<SecretsChange>> {
    if let Ok(chunk) = codec::deserialize::<SecretsChunkV1>(payload) {
        return wire_mapper::decode_chunk(chunk);
    }

    if let Ok(changes) = codec::deserialize::<Vec<SecretsChange>>(payload) {
        return Ok(changes);
    }

    if let Ok(maybe_change) = codec::deserialize::<Option<SecretsChange>>(payload) {
        return Ok(maybe_change.into_iter().collect());
    }

    Err(SecretError::DeserializationError(
        "Unable to decode secrets change payload".to_string(),
    ))
}
