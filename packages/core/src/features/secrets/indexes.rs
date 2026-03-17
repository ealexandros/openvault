use std::collections::HashMap;
use uuid::Uuid;

use super::error::{Result, SecretError};
use super::models::{LoginEntry, SecretFolder, SECRETS_ROOT_FOLDER_ID};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NameOwner {
    Folder(Uuid),
    Entry(Uuid),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SecretIndex {
    folders_by_parent: HashMap<Uuid, Vec<Uuid>>,
    entries_by_parent: HashMap<Uuid, Vec<Uuid>>,
    names: HashMap<(Uuid, String), NameOwner>,
}

impl SecretIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(
        folders: &HashMap<Uuid, SecretFolder>,
        entries: &HashMap<Uuid, LoginEntry>,
    ) -> Result<Self> {
        let mut index = Self::new();

        for folder in folders.values() {
            index.track_folder(folder)?;
        }

        for entry in entries.values() {
            index.track_entry(entry)?;
        }

        Ok(index)
    }

    pub fn folders(&self, parent_id: &Uuid) -> &[Uuid] {
        self.folders_by_parent
            .get(parent_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn entries(&self, parent_id: &Uuid) -> &[Uuid] {
        self.entries_by_parent
            .get(parent_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn track_folder(&mut self, folder: &SecretFolder) -> Result {
        if folder.id == SECRETS_ROOT_FOLDER_ID {
            return Ok(());
        }

        let parent_id = folder
            .parent_id
            .ok_or_else(|| SecretError::FolderMustHaveParent(folder.id))?;

        self.ensure_name_available(parent_id, &folder.name, Some(NameOwner::Folder(folder.id)))?;

        self.names.insert(
            (parent_id, folder.name.clone()),
            NameOwner::Folder(folder.id),
        );

        self.folders_by_parent
            .entry(parent_id)
            .or_default()
            .push(folder.id);

        Ok(())
    }

    pub fn untrack_folder(&mut self, folder: &SecretFolder) {
        if folder.id == SECRETS_ROOT_FOLDER_ID {
            return;
        }

        if let Some(parent_id) = folder.parent_id {
            self.names
                .remove(&(parent_id, folder.name.clone()));
            Self::remove_child(&mut self.folders_by_parent, parent_id, folder.id);
        }
    }

    pub fn rename_folder(&mut self, parent_id: Uuid, id: Uuid, old_name: &str, new_name: &str) {
        self.names.remove(&(parent_id, old_name.to_string()));
        self.names
            .insert((parent_id, new_name.to_string()), NameOwner::Folder(id));
    }

    pub fn track_entry(&mut self, entry: &LoginEntry) -> Result {
        self.ensure_name_available(
            entry.folder_id,
            &entry.name,
            Some(NameOwner::Entry(entry.id)),
        )?;

        self.names.insert(
            (entry.folder_id, entry.name.clone()),
            NameOwner::Entry(entry.id),
        );

        self.entries_by_parent
            .entry(entry.folder_id)
            .or_default()
            .push(entry.id);

        Ok(())
    }

    pub fn untrack_entry(&mut self, entry: &LoginEntry) {
        self.names
            .remove(&(entry.folder_id, entry.name.clone()));
        Self::remove_child(&mut self.entries_by_parent, entry.folder_id, entry.id);
    }

    pub fn ensure_entry_name_available(
        &self,
        parent_id: Uuid,
        name: &str,
        current_id: Option<Uuid>,
    ) -> Result {
        let current = current_id.map(NameOwner::Entry);
        self.ensure_name_available(parent_id, name, current)
    }

    pub fn ensure_folder_name_available(
        &self,
        parent_id: Uuid,
        name: &str,
        current_id: Option<Uuid>,
    ) -> Result {
        let current = current_id.map(NameOwner::Folder);
        self.ensure_name_available(parent_id, name, current)
    }

    fn ensure_name_available(
        &self,
        parent_id: Uuid,
        name: &str,
        current: Option<NameOwner>,
    ) -> Result {
        if let Some(existing) = self.names.get(&(parent_id, name.to_string())) {
            if Some(*existing) != current {
                return Err(SecretError::name_conflict(parent_id, name));
            }
        }

        Ok(())
    }

    fn remove_child(map: &mut HashMap<Uuid, Vec<Uuid>>, parent_id: Uuid, child_id: Uuid) {
        let Some(children) = map.get_mut(&parent_id) else {
            return;
        };

        children.retain(|&id| id != child_id);

        if children.is_empty() {
            map.remove(&parent_id);
        }
    }
}
