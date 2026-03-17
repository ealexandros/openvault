use std::collections::HashMap;

use uuid::Uuid;
use validator::Validate;
use zeroize::Zeroize;

use super::error::{Result, SecretError};
use super::indexes::SecretIndex;
use super::models::{LoginEntry, SecretFolder, SECRETS_ROOT_FOLDER_ID};
use super::patch::{LoginEntryPatch, SecretFolderPatch};
use super::records::{SecretDelta, SecretSnapshot, SecretsChange};
use super::validate;
use crate::features::shared::DEFAULT_SNAPSHOT_THRESHOLD;

#[derive(Clone, Debug)]
pub struct SecretStore {
    pub(crate) folders: HashMap<Uuid, SecretFolder>,
    pub(crate) entries: HashMap<Uuid, LoginEntry>,
    pub(crate) index: SecretIndex,
    pub(crate) deltas: Vec<SecretDelta>,
}

impl Default for SecretStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretStore {
    pub fn new() -> Self {
        let root = SecretFolder::root();
        let mut folders = HashMap::new();
        folders.insert(root.id, root);

        Self {
            folders,
            entries: HashMap::new(),
            index: SecretIndex::new(),
            deltas: Vec::new(),
        }
    }

    pub fn restore(snapshot: SecretSnapshot, deltas: Vec<SecretDelta>) -> Result<Self> {
        let index = SecretIndex::build(&snapshot.folders, &snapshot.entries)?;

        let mut store = Self {
            folders: snapshot.folders,
            entries: snapshot.entries,
            index,
            deltas: Vec::new(),
        };

        validate::validate_snapshot(&store.folders, &store.entries)?;

        for delta in &deltas {
            store.replay_delta(delta)?;
        }

        store.clear_deltas();

        Ok(store)
    }

    pub fn folder(&self, id: &Uuid) -> Option<&SecretFolder> {
        self.folders.get(id)
    }

    pub fn entry(&self, id: &Uuid) -> Option<&LoginEntry> {
        self.entries.get(id)
    }

    pub fn folders(&self, parent_id: Uuid) -> Vec<SecretFolder> {
        let folder_ids = self.index.folders(&parent_id);

        folder_ids
            .iter()
            .filter_map(|id| self.folders.get(id))
            .cloned()
            .collect()
    }

    pub fn entries(&self, parent_id: Uuid) -> Vec<LoginEntry> {
        let entry_ids = self.index.entries(&parent_id);

        entry_ids
            .iter()
            .filter_map(|id| self.entries.get(id))
            .cloned()
            .collect()
    }

    pub fn browse(&self, parent_id: &Uuid) -> Result<(Vec<SecretFolder>, Vec<LoginEntry>)> {
        if !self.folders.contains_key(parent_id) {
            return Err(SecretError::FolderNotFound(*parent_id));
        }

        Ok((self.folders(*parent_id), self.entries(*parent_id)))
    }

    pub fn add_folder(&mut self, parent_id: Uuid, name: String) -> Result<Uuid> {
        let folder = SecretFolder::new(Some(parent_id), name);
        let folder_id = folder.id;

        self.commit_delta(&SecretDelta::FolderAdded(folder))?;

        Ok(folder_id)
    }

    pub fn rename_folder(&mut self, id: Uuid, new_name: String) -> Result {
        let patch = SecretFolderPatch::rename(new_name);
        self.commit_delta(&SecretDelta::FolderUpdated { id, patch })
    }

    pub fn remove_folder(&mut self, id: Uuid) -> Result {
        self.commit_delta(&SecretDelta::FolderDeleted(id))
    }

    pub fn add_entry(&mut self, entry: LoginEntry) -> Result<Uuid> {
        let id = entry.id;
        self.commit_delta(&SecretDelta::EntryAdded(entry))?;
        Ok(id)
    }

    pub fn update_entry(&mut self, id: Uuid, patch: LoginEntryPatch) -> Result {
        self.commit_delta(&SecretDelta::EntryUpdated { id, patch })
    }

    pub fn move_entry(&mut self, id: Uuid, new_folder_id: Uuid) -> Result {
        let patch = LoginEntryPatch::move_to(new_folder_id);
        self.commit_delta(&SecretDelta::EntryUpdated { id, patch })
    }

    pub fn remove_entry(&mut self, id: Uuid) -> Result {
        self.commit_delta(&SecretDelta::EntryDeleted(id))
    }

    pub fn snapshot(&self) -> SecretSnapshot {
        SecretSnapshot::new(self.folders.clone(), self.entries.clone())
    }

    pub fn pending_changes(&self) -> Option<SecretsChange> {
        if self.deltas.is_empty() {
            return None;
        }

        if self.deltas.len() >= DEFAULT_SNAPSHOT_THRESHOLD {
            return Some(SecretsChange::Snapshot(self.snapshot()));
        }

        Some(SecretsChange::Deltas(self.deltas.clone()))
    }

    pub fn clear_deltas(&mut self) {
        self.deltas.clear();
    }

    pub fn apply_change(&mut self, change: SecretsChange) -> Result {
        match change {
            SecretsChange::Snapshot(snapshot) => self.replace_snapshot(snapshot)?,
            SecretsChange::Deltas(deltas) => {
                for delta in &deltas {
                    self.replay_delta(delta)?;
                }
            }
        }

        self.clear_deltas();
        Ok(())
    }

    fn replace_snapshot(&mut self, snapshot: SecretSnapshot) -> Result {
        self.folders = snapshot.folders;
        self.entries = snapshot.entries;
        self.index = SecretIndex::build(&self.folders, &self.entries)?;
        Ok(())
    }

    fn commit_delta(&mut self, delta: &SecretDelta) -> Result {
        self.apply_delta(delta, true)
    }

    fn replay_delta(&mut self, delta: &SecretDelta) -> Result {
        self.apply_delta(delta, false)
    }

    fn apply_delta(&mut self, delta: &SecretDelta, track_delta: bool) -> Result {
        match delta {
            SecretDelta::FolderAdded(folder) => self.apply_folder_added(folder.clone()),
            SecretDelta::FolderDeleted(id) => self.apply_folder_deleted(*id),
            SecretDelta::FolderUpdated { id, patch } => self.apply_folder_updated(*id, patch),
            SecretDelta::EntryAdded(entry) => self.apply_entry_added(entry.clone()),
            SecretDelta::EntryUpdated { id, patch } => self.apply_entry_updated(*id, patch),
            SecretDelta::EntryDeleted(id) => self.apply_entry_deleted(*id),
        }?;

        if track_delta {
            self.deltas.push(delta.clone());
        }

        Ok(())
    }

    fn apply_folder_added(&mut self, folder: SecretFolder) -> Result {
        if folder.id == SECRETS_ROOT_FOLDER_ID {
            return Err(SecretError::RootFolderReserved);
        }

        if self.folders.contains_key(&folder.id) || self.entries.contains_key(&folder.id) {
            return Err(SecretError::DuplicateId(folder.id));
        }

        let parent_id = folder
            .parent_id
            .ok_or_else(|| SecretError::FolderMustHaveParent(folder.id))?;

        if !self.folders.contains_key(&parent_id) {
            return Err(SecretError::ParentFolderNotFound(parent_id));
        }

        self.index
            .ensure_folder_name_available(parent_id, &folder.name, None)?;

        folder.validate()?;

        self.index.track_folder(&folder)?;
        self.folders.insert(folder.id, folder);

        Ok(())
    }

    fn apply_folder_deleted(&mut self, id: Uuid) -> Result {
        if id == SECRETS_ROOT_FOLDER_ID {
            return Err(SecretError::CannotDeleteRootFolder);
        }

        let entry_ids = self.index.entries(&id).to_vec();
        for entry_id in entry_ids {
            self.apply_entry_deleted(entry_id)?;
        }

        let folder_ids = self.index.folders(&id).to_vec();
        for folder_id in folder_ids {
            self.apply_folder_deleted(folder_id)?;
        }

        let folder = self
            .folders
            .get(&id)
            .ok_or(SecretError::FolderNotFound(id))?;

        let parent_id = folder
            .parent_id
            .ok_or(SecretError::FolderMissingParent(id))?;

        self.index.untrack_folder(folder);
        self.folders.remove(&id);

        if !self.folders.contains_key(&parent_id) {
            return Err(SecretError::ParentFolderNotFound(parent_id));
        }

        Ok(())
    }

    fn apply_folder_updated(&mut self, id: Uuid, patch: &SecretFolderPatch) -> Result {
        if id == SECRETS_ROOT_FOLDER_ID {
            return Err(SecretError::RootFolderImmutable);
        }

        let folder = self
            .folders
            .get(&id)
            .ok_or(SecretError::FolderNotFound(id))?;

        let parent_id = folder
            .parent_id
            .ok_or(SecretError::FolderMustHaveParent(id))?;

        let target_name = patch.name.clone().unwrap_or(folder.name.clone());

        if target_name != folder.name {
            self.index
                .ensure_folder_name_available(parent_id, &target_name, Some(id))?;
        }

        let mut candidate = folder.clone();
        candidate.name = target_name.clone();
        candidate.validate()?;

        if target_name != folder.name {
            self.index
                .rename_folder(parent_id, id, &folder.name, &target_name);
        }

        let folder = self.folders.get_mut(&id).expect("folder exists");
        folder.name = target_name;
        folder.updated_at = patch.updated_at;

        Ok(())
    }

    fn apply_entry_added(&mut self, entry: LoginEntry) -> Result {
        if self.entries.contains_key(&entry.id) || self.folders.contains_key(&entry.id) {
            return Err(SecretError::DuplicateId(entry.id));
        }

        if !self.folders.contains_key(&entry.folder_id) {
            return Err(SecretError::ParentFolderNotFound(entry.folder_id));
        }

        self.index
            .ensure_entry_name_available(entry.folder_id, &entry.name, None)?;

        entry.validate()?;

        self.index.track_entry(&entry)?;
        self.entries.insert(entry.id, entry);

        Ok(())
    }

    fn apply_entry_updated(&mut self, id: Uuid, patch: &LoginEntryPatch) -> Result {
        let current = self
            .entries
            .get(&id)
            .ok_or(SecretError::EntryNotFound(id))?;

        let target_folder = patch.folder_id.unwrap_or(current.folder_id);
        let target_name = patch.name.clone().unwrap_or(current.name.clone());

        if !self.folders.contains_key(&target_folder) {
            return Err(SecretError::ParentFolderNotFound(target_folder));
        }

        if target_folder != current.folder_id || target_name != current.name {
            self.index
                .ensure_entry_name_available(target_folder, &target_name, Some(id))?;
        }

        let old_entry = current.clone();
        let entry = self
            .entries
            .get_mut(&id)
            .ok_or(SecretError::EntryNotFound(id))?;

        entry.apply_patch(patch.clone())?;
        entry.validate()?;

        if target_folder != old_entry.folder_id || target_name != old_entry.name {
            self.index.untrack_entry(&old_entry);
            self.index.track_entry(entry)?;
        }

        Ok(())
    }

    fn apply_entry_deleted(&mut self, id: Uuid) -> Result {
        let entry = self
            .entries
            .remove(&id)
            .ok_or(SecretError::EntryNotFound(id))?;

        self.index.untrack_entry(&entry);

        Ok(())
    }
}

impl Zeroize for SecretStore {
    fn zeroize(&mut self) {
        for folder in self.folders.values_mut() {
            folder.zeroize();
        }
        self.folders.clear();

        for entry in self.entries.values_mut() {
            entry.zeroize();
        }
        self.entries.clear();

        for delta in &mut self.deltas {
            delta.zeroize();
        }
        self.deltas.clear();

        self.index = SecretIndex::new();
    }
}
