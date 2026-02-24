use std::collections::{BTreeSet, HashMap};

use uuid::Uuid;

use super::error::{Result, SecretError};
use super::indexes::StoreIndexes;
use super::models::{
    EncryptedField, LoginEntry, LoginEntryPatch, ROOT_FOLDER, normalize_folder_path,
};
use super::records::{SecretDelta, SecretSnapshot, SecretsChange};

const SNAPSHOT_THRESHOLD: usize = 30;

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
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

    pub fn restore(snapshot: SecretSnapshot, deltas: Vec<SecretDelta>) -> Result<Self> {
        let mut store = Self {
            entries: snapshot.entries,
            deltas: Vec::new(),
            ..Self::default()
        };

        store.indexes.rebuild(&store.entries)?;

        for delta in &deltas {
            store.apply_delta(delta, false)?;
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
        self.apply_delta(&SecretDelta::Added(entry), true)?;
        Ok(id)
    }

    pub fn update(&mut self, id: Uuid, patch: LoginEntryPatch) -> Result {
        self.apply_delta(&SecretDelta::Updated { id, patch }, true)
    }

    pub fn delete(&mut self, id: Uuid) -> Result {
        self.apply_delta(&SecretDelta::Deleted { id }, true)
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

    pub fn snapshot(&self) -> SecretSnapshot {
        SecretSnapshot::new(self.entries.clone())
    }

    pub fn create_snapshot(&self) -> SecretsChange {
        SecretsChange::Snapshot(self.snapshot())
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

    pub fn apply_change(&mut self, change: SecretsChange) -> Result {
        match change {
            SecretsChange::Snapshot(snapshot) => self.replace_snapshot(snapshot)?,
            SecretsChange::Deltas(deltas) => {
                for delta in &deltas {
                    self.apply_delta(delta, false)?;
                }
            }
        }

        self.reset_sync_state();
        Ok(())
    }

    fn replace_snapshot(&mut self, snapshot: SecretSnapshot) -> Result {
        self.entries = snapshot.entries;
        self.indexes.rebuild(&self.entries)
    }

    fn apply_delta(&mut self, delta: &SecretDelta, track_delta: bool) -> Result {
        match delta {
            SecretDelta::Added(entry) => {
                let mut entry = entry.clone();
                entry.folder = normalize_folder_path(&entry.folder);

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

        if track_delta {
            self.deltas.push(delta.clone());
        }

        Ok(())
    }
}

pub fn immediate_child_folder(parent: &str, candidate: &str) -> Option<String> {
    let parent = normalize_folder_path(parent);
    let candidate = normalize_folder_path(candidate);

    if parent == candidate {
        return None;
    }

    let prefix = if parent == ROOT_FOLDER {
        ROOT_FOLDER.to_string()
    } else {
        format!("{}/", parent)
    };

    if !candidate.starts_with(&prefix) {
        return None;
    }

    let remainder = candidate.strip_prefix(&prefix)?;
    if remainder.is_empty() {
        return None;
    }

    let next_segment = remainder.split('/').next()?;
    if next_segment.is_empty() {
        return None;
    }

    if parent == ROOT_FOLDER {
        Some(format!("/{}", next_segment))
    } else {
        Some(format!("{}/{}", parent, next_segment))
    }
}

use serde::{Deserialize, Serialize};
