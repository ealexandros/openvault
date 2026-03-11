use chrono::{DateTime, Utc};
use uuid::Uuid;

use openvault_core::features::messages::{
    MessageContact, MessageCredentials, MessageCredentialsView, MessagesStore,
};
use openvault_core::vault::runtime::VaultSession;

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

    pub fn credentials(&self) -> Option<MessageCredentialsView> {
        self.store.get_credentials()
    }

    pub fn create_credentials(
        &mut self,
        name: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<MessageCredentials> {
        self.store
            .create_credentials(name, expires_at)
            .map_err(Error::from)
    }

    pub fn renew_credentials(&mut self) -> Result<MessageCredentials> {
        self.store.renew_credentials().map_err(Error::from)
    }

    pub fn reset_credentials(&mut self) -> Result {
        self.store.reset_credentials().map_err(Error::from)
    }

    pub fn list_contacts(&self) -> Vec<MessageContact> {
        self.store.list_contacts()
    }

    pub fn add_contact(&mut self, contact: MessageContact) -> Result<Uuid> {
        self.store.add_contact(contact).map_err(Error::from)
    }

    pub fn rename_contact(&mut self, id: Uuid, new_name: String) -> Result {
        self.store.rename_contact(id, new_name).map_err(Error::from)
    }

    pub fn remove_contact(&mut self, id: Uuid) -> Result {
        self.store.remove_contact(id).map_err(Error::from)
    }

    pub fn find_contact(&self, id: &Uuid) -> Result<MessageContact> {
        self.store
            .find_contact(id)
            .ok_or_else(|| Error::ItemNotFound(id.to_string()))
    }

    pub fn encrypt_for_contact(&self, id: Uuid, payload: &[u8]) -> Result<String> {
        self.store
            .encrypt_for_contact(id, payload)
            .map_err(Error::from)
    }

    pub fn encrypt_for_contact_name(&self, name: &str, payload: &[u8]) -> Result<String> {
        self.store
            .encrypt_for_contact_name(name, payload)
            .map_err(Error::from)
    }

    pub fn decrypt_from_contact(&self, id: Uuid, payload: &[u8]) -> Result<String> {
        self.store
            .decrypt_from_contact(id, payload)
            .map_err(Error::from)
    }

    pub fn decrypt_from_contact_name(&self, name: &str, payload: &[u8]) -> Result<String> {
        self.store
            .decrypt_from_contact_name(name, payload)
            .map_err(Error::from)
    }
}
