pub mod params;

pub mod codec;

use openvault_crypto::keys::MasterKey;

use crate::domain::entry::{EncryptedField, SecretEntryView};
use crate::domain::store::{SecretsChange, SecretsStore, Snapshot};
use crate::errors::{Result, SecretError};
use crate::manager::params::{AddSecretEntryParams, UpdateSecretEntryParams};

// @todo-soon consider adding derived key for data fields

#[derive(Debug)]
pub struct SecretManager {
    key: MasterKey,
    store: SecretsStore,
}

impl SecretManager {
    pub fn unlock(key: MasterKey, chunks: Vec<Vec<u8>>) -> Result<SecretManager> {
        if chunks.is_empty() {
            return Ok(SecretManager::create(key));
        }

        let store = SecretManager::restore_store(&key, &chunks)?;

        Ok(Self { key, store })
    }

    fn restore_store(key: &MasterKey, chunks: &[Vec<u8>]) -> Result<SecretsStore> {
        let mut snapshot = Snapshot::default();
        let mut deltas = Vec::new();

        for chunk in chunks {
            let decrypted = codec::decrypt(chunk, key)?;
            let changes: Vec<SecretsChange> = codec::deserialize(&decrypted)?;

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

        SecretsStore::restore(snapshot, deltas)
    }
}

impl SecretManager {
    pub fn create(key: MasterKey) -> Self {
        Self {
            key,
            store: SecretsStore::new(),
        }
    }

    pub fn list(&self) -> Vec<SecretEntryView> {
        self.store.list()
    }

    pub fn get(&self, name: &str) -> Option<SecretEntryView> {
        self.store.get(name)
    }

    pub fn reveal_password(&self, name: &str) -> Result<String> {
        let encrypted = self
            .store
            .get_encrypted_password(name)
            .ok_or_else(|| SecretError::NotFound(name.to_string()))?;

        codec::decrypt_password(encrypted.as_bytes(), &self.key)
    }

    pub fn add(&mut self, params: AddSecretEntryParams) -> Result {
        let entry = params.into_entry(&self.key)?;
        self.store.add(entry)
    }

    pub fn update(&mut self, name: String, params: UpdateSecretEntryParams) -> Result {
        let patch = params.into_patch(&self.key)?;
        self.store.update(name, patch)
    }

    pub fn delete(&mut self, name: String) -> Result {
        self.store.delete(name)
    }

    pub fn export(&self) -> Result<Vec<u8>> {
        let snapshot = self.store.snapshot();
        let serialized = codec::serialize(&snapshot)?;
        codec::encrypt(&serialized, &self.key)
    }

    pub fn export_changes(&self) -> Result<Vec<u8>> {
        let changes = self.store.pending_change();
        let serialized = codec::serialize(&changes)?;
        codec::encrypt(&serialized, &self.key)
    }

    pub fn clear_deltas(&mut self) {
        self.store.clear_deltas()
    }

    pub fn lock(self) {
        drop(self);
    }
}
