pub mod codec;
pub mod params;

use openvault_crypto::keys::MasterKey;
use uuid::Uuid;

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

// @todo-now fix the uuid and the where it is initialized

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

    pub fn get(&self, id: &Uuid) -> Option<SecretEntryView> {
        self.store.get(id)
    }

    pub fn reveal_password(&self, id: &Uuid) -> Result<String> {
        let encrypted = self
            .store
            .get_encrypted_password(id)
            .ok_or_else(|| SecretError::NotFound(id.to_string()))?;

        codec::decrypt_password(encrypted.as_bytes(), &self.key)
    }

    pub fn add(&mut self, params: AddSecretEntryParams) -> Result<Uuid> {
        let entry = params.into_entry(&self.key)?;
        let id = entry.id;
        self.store.add(entry)?;
        Ok(id)
    }

    pub fn update(&mut self, id: &Uuid, params: UpdateSecretEntryParams) -> Result {
        let patch = params.into_patch(&self.key)?;
        self.store.update(*id, patch)
    }

    pub fn delete(&mut self, id: &Uuid) -> Result {
        self.store.delete(*id)
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
