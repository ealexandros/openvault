use chrono::{DateTime, Utc};
use uuid::Uuid;

use openvault_core::features::messages::{
    MessageContact, MessageContactPatch, MessageCredentials, MessagesStore,
};
use openvault_core::repositories::{FeatureRepository, MessagesRepository};
use openvault_core::vault::runtime::VaultSession;

use crate::errors::{Error, Result};

// @todo-now fix these..

pub struct MessagesService<'a> {
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

    pub fn set_credentials(
        &mut self,
        name: String,
        public_key: Vec<u8>,
        private_key: Vec<u8>,
    ) -> Result {
        let credentials = MessageCredentials::new(name, public_key, private_key)?;
        self.store.set_credentials(credentials).map_err(Error::from)
    }

    pub fn clear_credentials(&mut self) -> Result {
        self.store.clear_credentials().map_err(Error::from)
    }

    pub fn add_contact(
        &mut self,
        name: String,
        expiration_at: Option<DateTime<Utc>>,
        secure: bool,
        public_key: Vec<u8>,
    ) -> Result<Uuid> {
        self.store
            .add_contact(name, public_key, secure, expiration_at)
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
            .contact(id)
            .ok_or_else(|| Error::ItemNotFound(id.to_string()))
    }

    pub fn reload(&mut self) -> Result<&MessagesStore> {
        *self.store = MessagesRepository::load(self.session)?;
        Ok(self.store)
    }
}
