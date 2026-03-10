use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;
use zeroize::Zeroize;

use openvault_crypto::keys::{
    EphemeralKeyPair, EphemeralPublicKey, SigningKeyPair, SigningPublicKey,
};
use validator::Validate;

use crate::features::shared::DEFAULT_SNAPSHOT_THRESHOLD;

use super::crypto::{MessageEnvelope, open_message, seal_message};
use super::error::{MessagesError, Result};
use super::events::{MessagesChange, MessagesDelta, MessagesSnapshot};
use super::models::{MessageContact, MessageContactPatch, MessageCredentials};

#[derive(Clone, Debug, Default)]
pub struct MessagesStore {
    contacts: HashMap<Uuid, MessageContact>,
    credentials: Option<MessageCredentials>,
    deltas: Vec<MessagesDelta>,
}

// @todo-now don't return the private keys..

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
            store.replay_delta(delta)?;
        }
        store.clear_deltas();

        // @todo-now validate if credentials are set if not no users must be present

        Ok(store)
    }

    pub fn credentials(&self) -> Option<&MessageCredentials> {
        self.credentials.as_ref()
    }

    pub fn find_contact(&self, id: &Uuid) -> Option<MessageContact> {
        self.contacts.get(id).cloned()
    }

    pub fn list_contacts(&self) -> Vec<MessageContact> {
        let mut contacts: Vec<MessageContact> = self.contacts.values().cloned().collect();
        contacts.sort_by(|a, b| a.name.cmp(&b.name));
        contacts
    }

    pub fn create_credentials(
        &mut self,
        name: String,
        expiration_at: Option<DateTime<Utc>>,
    ) -> Result<MessageCredentials> {
        let credentials = Self::build_credentials(name, expiration_at)?;
        self.set_credentials(credentials.clone())?;
        Ok(credentials)
    }

    pub fn renew_credentials(&mut self) -> Result<MessageCredentials> {
        let current = self
            .credentials
            .as_ref()
            .ok_or(MessagesError::CredentialsNotSet)?;

        let renewed = Self::build_credentials(current.name.clone(), current.expiration_at)?;
        self.set_credentials(renewed.clone())?;

        Ok(renewed)
    }

    pub fn set_credentials(&mut self, credentials: MessageCredentials) -> Result {
        self.commit_delta(&MessagesDelta::CredentialsSet(credentials))
    }

    pub fn clear_credentials(&mut self) -> Result {
        self.commit_delta(&MessagesDelta::CredentialsCleared)
    }

    pub fn add_contact(
        &mut self,
        name: String,
        signing_public_key: SigningPublicKey,
        ephemeral_public_key: EphemeralPublicKey,
        secure: bool,
        expiration_at: Option<DateTime<Utc>>,
    ) -> Result<Uuid> {
        let contact = MessageContact {
            id: Uuid::new_v4(),
            name,
            expiration_at,
            secure,
            signing_key: signing_public_key,
            ephemeral_key: ephemeral_public_key,
        };
        let id = contact.id;
        self.commit_delta(&MessagesDelta::ContactAdded(contact))?;
        Ok(id)
    }

    pub fn update_contact(&mut self, id: Uuid, patch: MessageContactPatch) -> Result {
        self.commit_delta(&MessagesDelta::ContactUpdated { id, patch })
    }

    pub fn remove_contact(&mut self, id: Uuid) -> Result {
        self.commit_delta(&MessagesDelta::ContactDeleted(id))
    }

    pub fn encrypt_for_contact(&self, id: Uuid, message: &[u8]) -> Result<MessageEnvelope> {
        let credentials = self.ensure_credentials()?;
        let contact = self
            .contacts
            .get(&id)
            .ok_or_else(|| MessagesError::NotFound(id.to_string()))?;

        seal_message(message, credentials, &contact.ephemeral_key)
    }

    pub fn decrypt_from_contact(&self, id: Uuid, envelope: &MessageEnvelope) -> Result<Vec<u8>> {
        let credentials = self.ensure_credentials()?;
        let contact = self
            .contacts
            .get(&id)
            .ok_or_else(|| MessagesError::NotFound(id.to_string()))?;

        open_message(envelope, credentials, &contact.signing_key)
    }

    pub fn encrypt_for_contact_name(&self, name: &str, message: &[u8]) -> Result<MessageEnvelope> {
        let contact = self.contact_by_name(name)?;
        self.encrypt_for_contact(contact.id, message)
    }

    pub fn decrypt_from_contact_name(
        &self,
        name: &str,
        envelope: &MessageEnvelope,
    ) -> Result<Vec<u8>> {
        let contact = self.contact_by_name(name)?;
        self.decrypt_from_contact(contact.id, envelope)
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

    fn ensure_credentials(&self) -> Result<&MessageCredentials> {
        self.credentials
            .as_ref()
            .ok_or(MessagesError::CredentialsNotSet)
    }

    fn contact_by_name(&self, name: &str) -> Result<&MessageContact> {
        self.contacts
            .values()
            .find(|contact| contact.name == name)
            .ok_or_else(|| MessagesError::NotFound(name.to_string()))
    }

    fn build_credentials(
        name: String,
        expiration_at: Option<DateTime<Utc>>,
    ) -> Result<MessageCredentials> {
        let credentials = MessageCredentials {
            name,
            signing_keys: SigningKeyPair::generate(),
            ephemeral_keys: EphemeralKeyPair::generate(),
            expiration_at,
        };

        credentials.validate()?;

        Ok(credentials)
    }

    fn commit_delta(&mut self, delta: &MessagesDelta) -> Result {
        self.apply_delta(delta, true)
    }

    fn replay_delta(&mut self, delta: &MessagesDelta) -> Result {
        self.apply_delta(delta, false)
    }

    fn apply_delta(&mut self, delta: &MessagesDelta, track_delta: bool) -> Result {
        match delta {
            MessagesDelta::ContactAdded(contact) => self.apply_contact_added(contact.clone())?,
            MessagesDelta::ContactUpdated { id, patch } => {
                self.apply_contact_updated(*id, patch.clone())?
            }
            MessagesDelta::ContactDeleted(id) => self.apply_contact_deleted(*id)?,
            MessagesDelta::CredentialsSet(credentials) => {
                self.apply_credentials_set(credentials.clone())?
            }
            MessagesDelta::CredentialsCleared => self.apply_credentials_cleared()?,
        }

        if track_delta {
            self.deltas.push(delta.clone());
        }

        Ok(())
    }

    fn apply_contact_added(&mut self, contact: MessageContact) -> Result {
        contact.validate()?;

        if self.contacts.contains_key(&contact.id) {
            return Err(MessagesError::DuplicateId(contact.id));
        }

        self.contacts.insert(contact.id, contact);

        Ok(())
    }

    fn apply_contact_updated(&mut self, id: Uuid, patch: MessageContactPatch) -> Result {
        patch.validate()?;

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

        if let Some(signing_public_key) = patch.signing_key {
            contact.signing_key = signing_public_key;
        }

        if let Some(ephemeral_public_key) = patch.ephemeral_key {
            contact.ephemeral_key = ephemeral_public_key;
        }

        contact.validate()?;

        Ok(())
    }

    fn apply_contact_deleted(&mut self, id: Uuid) -> Result {
        self.contacts
            .remove(&id)
            .ok_or_else(|| MessagesError::NotFound(id.to_string()))?;
        Ok(())
    }

    fn apply_credentials_set(&mut self, credentials: MessageCredentials) -> Result {
        credentials.validate()?;
        self.credentials = Some(credentials);
        Ok(())
    }

    fn apply_credentials_cleared(&mut self) -> Result {
        self.credentials = None;
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
