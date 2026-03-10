use chrono::{DateTime, Utc};
use uuid::Uuid;

use openvault_core::features::messages::{
    MessageContact, MessageContactPatch, MessageCredentials, MessageEnvelope, MessagesStore,
};
use openvault_core::vault::runtime::VaultSession;
use openvault_crypto::keys::{EphemeralPublicKey, SigningPublicKey};

use crate::errors::{Error, Result};

pub struct MessagesService<'a> {
    #[allow(dead_code)]
    session: &'a mut VaultSession,
    store: &'a mut MessagesStore,
}

impl<'a> MessagesService<'a> {
    pub fn new(session: &'a mut VaultSession, store: &'a mut MessagesStore) -> Self {
        Self { session, store }
    }

    pub fn credentials(&self) -> Option<MessageCredentials> {
        self.store.credentials().cloned()
    }

    pub fn set_credentials(&mut self, credentials: MessageCredentials) -> Result {
        self.store.set_credentials(credentials).map_err(Error::from)
    }

    pub fn create_credentials(
        &mut self,
        name: String,
        expiration_at: Option<DateTime<Utc>>,
    ) -> Result<MessageCredentials> {
        self.store
            .create_credentials(name, expiration_at)
            .map_err(Error::from)
    }

    pub fn renew_credentials(&mut self) -> Result<MessageCredentials> {
        self.store.renew_credentials().map_err(Error::from)
    }

    pub fn clear_credentials(&mut self) -> Result {
        self.store.clear_credentials().map_err(Error::from)
    }

    pub fn add_contact(
        &mut self,
        name: String,
        signing_public_key: SigningPublicKey,
        ephemeral_public_key: EphemeralPublicKey,
        secure: bool,
        expiration_at: Option<DateTime<Utc>>,
    ) -> Result<Uuid> {
        self.store
            .add_contact(
                name,
                signing_public_key,
                ephemeral_public_key,
                secure,
                expiration_at,
            )
            .map_err(Error::from)
    }

    pub fn update_contact(&mut self, id: Uuid, patch: MessageContactPatch) -> Result {
        self.store.update_contact(id, patch).map_err(Error::from)
    }

    pub fn remove_contact(&mut self, id: Uuid) -> Result {
        self.store.remove_contact(id).map_err(Error::from)
    }

    pub fn list_contacts(&self) -> Vec<MessageContact> {
        self.store.list_contacts()
    }

    pub fn get_contact(&self, id: &Uuid) -> Result<MessageContact> {
        self.store
            .find_contact(id)
            .ok_or_else(|| Error::ItemNotFound(id.to_string()))
    }

    pub fn encrypt_for_contact(&self, id: Uuid, message: &[u8]) -> Result<MessageEnvelope> {
        self.store
            .encrypt_for_contact(id, message)
            .map_err(Error::from)
    }

    pub fn decrypt_from_contact(&self, id: Uuid, envelope: &MessageEnvelope) -> Result<Vec<u8>> {
        self.store
            .decrypt_from_contact(id, envelope)
            .map_err(Error::from)
    }

    pub fn encrypt_for_contact_name(&self, name: &str, message: &[u8]) -> Result<MessageEnvelope> {
        self.store
            .encrypt_for_contact_name(name, message)
            .map_err(Error::from)
    }

    pub fn decrypt_from_contact_name(
        &self,
        name: &str,
        envelope: &MessageEnvelope,
    ) -> Result<Vec<u8>> {
        self.store
            .decrypt_from_contact_name(name, envelope)
            .map_err(Error::from)
    }
}
