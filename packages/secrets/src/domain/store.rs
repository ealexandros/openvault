use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use uuid::Uuid;

use crate::domain::folders::{immediate_child_folder, normalize_folder_path};
use crate::domain::indexes::StoreIndexes;
use crate::domain::records::{SecretDelta, SecretsChange, Snapshot};
use crate::domain::secrets::crypto::EncryptedField;
use crate::domain::secrets::login::{LoginEntry, LoginEntryPatch};
use crate::errors::{Result, SecretError};

const SNAPSHOT_THRESHOLD: usize = 30;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SecretStore {
    entries: HashMap<Uuid, LoginEntry>,
    deltas: Vec<SecretDelta>,
    #[serde(skip, default)]
    indexes: StoreIndexes,
}

impl SecretStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn restore(snapshot: Snapshot, deltas: Vec<SecretDelta>) -> Result<Self> {
        let mut store = Self {
            entries: snapshot.entries,
            ..Self::default()
        };

        store.indexes.rebuild(&store.entries)?;

        for delta in &deltas {
            store.apply_delta(delta)?;
        }

        store.reset_sync_state();
        Ok(store)
    }

    pub fn get_entry(&self, id: &Uuid) -> Option<LoginEntry> {
        self.entries.get(id).cloned()
    }

    pub fn show_password(&self, id: &Uuid) -> Option<EncryptedField> {
        self.entries.get(id).map(|e| e.password.clone())
    }

    pub fn insert(&mut self, entry: LoginEntry) -> Result<Uuid> {
        let id = entry.id;
        self.commit_delta(SecretDelta::Added(entry))?;
        Ok(id)
    }

    pub fn update(&mut self, id: Uuid, patch: LoginEntryPatch) -> Result {
        self.commit_delta(SecretDelta::Updated { id, patch })
    }

    pub fn delete(&mut self, id: Uuid) -> Result {
        self.commit_delta(SecretDelta::Deleted { id })
    }

    pub fn list_all(&self) -> Vec<LoginEntry> {
        let mut entries: Vec<LoginEntry> = self.entries.values().cloned().collect();
        entries.sort_by(|a, b| {
            a.folder
                .cmp(&b.folder)
                .then_with(|| a.name.cmp(&b.name))
                .then_with(|| a.username.cmp(&b.username))
        });
        entries
    }

    pub fn list_by_folder(&self, folder: &str) -> Vec<LoginEntry> {
        let current_path = normalize_folder_path(folder);
        let mut entries: Vec<LoginEntry> = self
            .indexes
            .entry_ids_in_folder(&current_path)
            .into_iter()
            .flatten()
            .filter_map(|id| self.entries.get(id))
            .cloned()
            .collect();

        entries.sort_by(|a, b| {
            a.name
                .cmp(&b.name)
                .then_with(|| a.username.cmp(&b.username))
        });

        entries
    }

    pub fn list_subfolders(&self, folder: &str) -> BTreeSet<String> {
        let current_path = normalize_folder_path(folder);
        self.indexes
            .folder_paths()
            .filter_map(|path| immediate_child_folder(&current_path, path))
            .collect::<BTreeSet<_>>()
    }

    pub fn create_snapshot(&self) -> SecretsChange {
        SecretsChange::Snapshot(Snapshot::new(self.entries.clone()))
    }

    pub fn pending_changes(&self) -> Option<SecretsChange> {
        if self.deltas.is_empty() {
            return None;
        }

        if self.deltas.len() >= SNAPSHOT_THRESHOLD {
            return Some(self.create_snapshot());
        }

        Some(SecretsChange::Deltas(self.deltas.clone()))
    }

    pub fn reset_sync_state(&mut self) {
        self.deltas.clear();
    }

    fn commit_delta(&mut self, delta: SecretDelta) -> Result {
        self.apply_delta(&delta)?;
        self.deltas.push(delta);
        Ok(())
    }

    fn apply_delta(&mut self, delta: &SecretDelta) -> Result {
        match delta {
            SecretDelta::Added(entry) => {
                let mut entry = entry.clone();
                entry.folder = normalize_folder_path(&entry.folder);

                self.indexes
                    .ensure_name_available(&entry.folder, &entry.name, None)?;
                self.indexes.track_entry(&entry)?;
                self.entries.insert(entry.id, entry);
            }
            SecretDelta::Updated { id, patch } => {
                let current = self
                    .entries
                    .get(id)
                    .ok_or_else(|| SecretError::NotFound(id.to_string()))?;

                let target_folder = patch
                    .folder
                    .as_deref()
                    .map(normalize_folder_path)
                    .unwrap_or_else(|| current.folder.clone());

                let target_name = patch.name.as_ref().unwrap_or(&current.name);

                self.indexes
                    .ensure_name_available(&target_folder, target_name, Some(id))?;

                let old_entry = current.clone();
                let entry = self.entries.get_mut(id).unwrap();

                entry.patch(patch.clone())?;
                entry.folder = normalize_folder_path(&entry.folder);

                self.indexes.untrack_entry(&old_entry);
                self.indexes.track_entry(entry)?;
            }
            SecretDelta::Deleted { id } => {
                let entry = self
                    .entries
                    .remove(id)
                    .ok_or_else(|| SecretError::NotFound(id.to_string()))?;
                self.indexes.untrack_entry(&entry);
            }
        }

        Ok(())
    }
}
