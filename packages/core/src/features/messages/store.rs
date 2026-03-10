use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;
use zeroize::Zeroize;

use crate::features::shared::DEFAULT_SNAPSHOT_THRESHOLD;

use super::error::{MessagesError, Result};
use super::events::{MessagesChange, MessagesDelta, MessagesSnapshot};
use super::models::{MessageContact, MessageContactPatch, MessageCredentials};

#[derive(Clone, Debug, Default)]
pub struct MessagesStore {
    contacts: HashMap<Uuid, MessageContact>,
    credentials: Option<MessageCredentials>,
    deltas: Vec<MessagesDelta>,
}

impl MessagesStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn restore(snapshot: MessagesSnapshot, deltas: Vec<MessagesDelta>) -> Result<Self> {
        let mut store = Self {
            contacts: snapshot.contacts,
            credentials: snapshot.credentials,
            deltas: Vec::new(),
        };

        for delta in &deltas {
            store.apply_delta(delta, false)?;
        }

        store.clear_deltas();

        Ok(store)
    }

    pub fn credentials(&self) -> Option<&MessageCredentials> {
        self.credentials.as_ref()
    }

    pub fn contact(&self, id: &Uuid) -> Option<MessageContact> {
        self.contacts.get(id).cloned()
    }

    pub fn list_contacts(&self) -> Vec<MessageContact> {
        let mut contacts: Vec<MessageContact> = self.contacts.values().cloned().collect();
        contacts.sort_by(|a, b| a.name.cmp(&b.name));
        contacts
    }

    pub fn set_credentials(&mut self, credentials: MessageCredentials) -> Result {
        self.apply_delta(&MessagesDelta::CredentialsSet(credentials), true)
    }

    pub fn clear_credentials(&mut self) -> Result {
        self.apply_delta(&MessagesDelta::CredentialsCleared, true)
    }

    pub fn add_contact(
        &mut self,
        name: String,
        public_key: Vec<u8>,
        secure: bool,
        expiration_at: Option<DateTime<Utc>>,
    ) -> Result<Uuid> {
        let contact = MessageContact::new(name, public_key, secure, expiration_at)?;
        let id = contact.id;
        self.apply_delta(&MessagesDelta::ContactAdded(contact), true)?;
        Ok(id)
    }

    pub fn update_contact(&mut self, id: Uuid, patch: MessageContactPatch) -> Result {
        self.apply_delta(&MessagesDelta::ContactUpdated { id, patch }, true)
    }

    pub fn remove_contact(&mut self, id: Uuid) -> Result {
        self.apply_delta(&MessagesDelta::ContactDeleted(id), true)
    }

    pub fn snapshot(&self) -> MessagesSnapshot {
        MessagesSnapshot::new(self.credentials.clone(), self.contacts.clone())
    }

    pub fn pending_changes(&self) -> Option<MessagesChange> {
        if self.deltas.is_empty() {
            return None;
        }

        if self.deltas.len() >= DEFAULT_SNAPSHOT_THRESHOLD {
            return Some(MessagesChange::Snapshot(self.snapshot()));
        }

        Some(MessagesChange::Deltas(self.deltas.clone()))
    }

    pub fn clear_deltas(&mut self) {
        self.deltas.clear();
    }

    pub fn apply_change(&mut self, change: MessagesChange) -> Result {
        match change {
            MessagesChange::Snapshot(snapshot) => self.replace_snapshot(snapshot),
            MessagesChange::Deltas(deltas) => {
                for delta in &deltas {
                    self.apply_delta(delta, false)?;
                }
                Ok(())
            }
        }?;

        self.clear_deltas();
        Ok(())
    }

    fn replace_snapshot(&mut self, snapshot: MessagesSnapshot) -> Result {
        self.contacts = snapshot.contacts;
        self.credentials = snapshot.credentials;
        Ok(())
    }

    fn apply_delta(&mut self, delta: &MessagesDelta, track_delta: bool) -> Result {
        match delta {
            MessagesDelta::ContactAdded(contact) => {
                if self.contacts.contains_key(&contact.id) {
                    return Err(MessagesError::DuplicateId(contact.id));
                }

                self.contacts.insert(contact.id, contact.clone());
            }
            MessagesDelta::ContactUpdated { id, patch } => {
                self.patch(*id, patch.clone())?;
            }
            MessagesDelta::ContactDeleted(id) => {
                self.contacts
                    .remove(id)
                    .ok_or_else(|| MessagesError::NotFound(id.to_string()))?;
            }
            MessagesDelta::CredentialsSet(credentials) => {
                self.credentials = Some(credentials.clone());
            }
            MessagesDelta::CredentialsCleared => {
                self.credentials = None;
            }
        }

        if track_delta {
            self.deltas.push(delta.clone());
        }

        Ok(())
    }

    pub fn patch(&mut self, id: Uuid, patch: MessageContactPatch) -> Result {
        let contact = self
            .contacts
            .get_mut(&id)
            .ok_or_else(|| MessagesError::NotFound(id.to_string()))?;

        if let Some(name) = patch.name {
            contact.name = name;
        }
        if let Some(expiration_at) = patch.expiration_at {
            contact.expiration_at = expiration_at;
        }
        if let Some(secure) = patch.secure {
            contact.secure = secure;
        }
        if let Some(public_key) = patch.public_key {
            contact.public_key = public_key;
        }

        Ok(())
    }
}

impl Zeroize for MessagesStore {
    fn zeroize(&mut self) {
        for contact in self.contacts.values_mut() {
            contact.zeroize();
        }
        self.contacts.clear();

        if let Some(credentials) = &mut self.credentials {
            credentials.zeroize();
        }
        self.credentials = None;

        for delta in &mut self.deltas {
            delta.zeroize();
        }
        self.deltas.clear();
    }
}
