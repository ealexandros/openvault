use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::entry::{EncryptedField, SecretEntry, SecretEntryPatch, SecretEntryView};
use crate::errors::{Result, SecretError};

// @todo-soon consider restructure for storage like API keys, notes, ssh keys.

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Snapshot {
    pub entries: HashMap<String, SecretEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecretDelta {
    Added(SecretEntry),
    Updated {
        name: String,
        patch: SecretEntryPatch,
    },
    Deleted {
        name: String,
    },
}

impl Snapshot {
    pub fn new(entries: HashMap<String, SecretEntry>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecretsChange {
    Snapshot(Snapshot),
    Deltas(Vec<SecretDelta>),
}

const SNAPSHOT_INTERVAL: usize = 30;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SecretsStore {
    entries: HashMap<String, SecretEntry>,
    deltas: Vec<SecretDelta>,
    deltas_count: usize,
}

impl SecretsStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn restore(snapshot: Snapshot, deltas: Vec<SecretDelta>) -> Result<Self> {
        let mut store = Self {
            entries: snapshot.entries,
            deltas: vec![],
            deltas_count: deltas.len(),
        };

        for delta in &deltas {
            store.apply_delta(delta)?;
        }

        Ok(store)
    }

    fn apply_delta(&mut self, delta: &SecretDelta) -> Result {
        match delta {
            SecretDelta::Added(entry) => {
                if self.entries.contains_key(&entry.name) {
                    return Err(SecretError::AlreadyExists(entry.name.clone()));
                }
                self.entries.insert(entry.name.clone(), entry.clone());
            }
            SecretDelta::Updated { name, patch } => {
                let entry = self
                    .entries
                    .get_mut(name)
                    .ok_or_else(|| SecretError::NotFound(name.clone()))?;
                entry.patch(patch.clone())?;
            }
            SecretDelta::Deleted { name } => {
                self.entries
                    .remove(name)
                    .ok_or_else(|| SecretError::InvalidName(name.clone()))?;
            }
        }
        Ok(())
    }

    fn record(&mut self, delta: SecretDelta) -> Result {
        self.apply_delta(&delta)?;
        self.deltas.push(delta);
        self.deltas_count += 1;
        Ok(())
    }

    pub fn add(&mut self, entry: SecretEntry) -> Result {
        self.record(SecretDelta::Added(entry))
    }

    pub fn update(&mut self, name: String, patch: SecretEntryPatch) -> Result {
        self.record(SecretDelta::Updated { name, patch })
    }

    pub fn delete(&mut self, name: String) -> Result {
        self.record(SecretDelta::Deleted { name })
    }

    pub fn get(&self, name: &str) -> Option<SecretEntryView> {
        self.entries.get(name).map(|e| e.into())
    }

    pub fn get_encrypted_password(&self, name: &str) -> Option<EncryptedField> {
        self.entries.get(name).map(|e| e.password.clone())
    }

    pub fn list(&self) -> Vec<SecretEntryView> {
        self.entries.values().map(|e| e.into()).collect()
    }

    pub fn pending_change(&self) -> Option<SecretsChange> {
        if self.deltas_count == 0 {
            return None;
        }

        if self.deltas_count >= SNAPSHOT_INTERVAL {
            return Some(self.snapshot());
        }

        Some(SecretsChange::Deltas(self.deltas.clone()))
    }

    pub fn snapshot(&self) -> SecretsChange {
        SecretsChange::Snapshot(Snapshot::new(self.entries.clone()))
    }

    pub fn clear_deltas(&mut self) {
        self.deltas.clear();
        self.deltas_count = 0;
    }

    pub fn entries(&self) -> &HashMap<String, SecretEntry> {
        &self.entries
    }
}
