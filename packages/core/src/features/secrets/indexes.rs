use std::collections::hash_map::Keys;
use std::collections::{BTreeSet, HashMap};
use uuid::Uuid;

use super::error::{Result, SecretError};
use super::models::{LoginEntry, normalize_folder_path};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct StoreIndexes {
    folder_entries: HashMap<String, BTreeSet<Uuid>>,
    entry_names: HashMap<(String, String), Uuid>,
}

impl StoreIndexes {
    pub(crate) fn rebuild(&mut self, entries: &HashMap<Uuid, LoginEntry>) -> Result {
        self.folder_entries.clear();
        self.entry_names.clear();

        for entry in entries.values() {
            self.track_entry(entry)?;
        }

        Ok(())
    }

    pub(crate) fn track_entry(&mut self, entry: &LoginEntry) -> Result {
        let folder = normalize_folder_path(&entry.folder);
        self.ensure_name_available(&folder, &entry.name, Some(&entry.id))?;

        self.entry_names
            .insert((folder.clone(), entry.name.clone()), entry.id);
        self.folder_entries
            .entry(folder)
            .or_default()
            .insert(entry.id);

        Ok(())
    }

    pub(crate) fn untrack_entry(&mut self, entry: &LoginEntry) {
        let folder = normalize_folder_path(&entry.folder);
        self.entry_names
            .remove(&(folder.clone(), entry.name.clone()));

        let should_remove = if let Some(entry_ids) = self.folder_entries.get_mut(&folder) {
            entry_ids.remove(&entry.id);
            entry_ids.is_empty()
        } else {
            false
        };

        if should_remove {
            self.folder_entries.remove(&folder);
        }
    }

    pub(crate) fn ensure_name_available(
        &self,
        folder: &str,
        name: &str,
        current_id: Option<&Uuid>,
    ) -> Result {
        if let Some(existing_id) = self
            .entry_names
            .get(&(folder.to_string(), name.to_string()))
        {
            let is_same_entry = current_id.is_some_and(|id| id == existing_id);
            if !is_same_entry {
                return Err(SecretError::AlreadyExists(name.to_string()));
            }
        }

        Ok(())
    }

    pub(crate) fn entry_ids_in_folder(&self, folder: &str) -> Option<&BTreeSet<Uuid>> {
        self.folder_entries.get(folder)
    }

    pub(crate) fn folder_paths(&self) -> Keys<'_, String, BTreeSet<Uuid>> {
        self.folder_entries.keys()
    }
}
