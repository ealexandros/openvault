use uuid::Uuid;

use openvault_core::features::secrets::{
    LoginEntry, LoginEntryPatch, LoginEntryView, SecretFolder, SecretStore, TOTP,
    SECRETS_FEATURE_ID,
};
use openvault_core::vault::runtime::VaultSession;
use openvault_crypto::keys::derived_key::DerivedKey;

use crate::errors::{Error, Result};

pub struct SecretsService<'a> {
    session: &'a mut VaultSession,
    store: &'a mut SecretStore,
}

impl<'a> SecretsService<'a> {
    pub fn new(session: &'a mut VaultSession, store: &'a mut SecretStore) -> Self {
        Self { session, store }
    }

    pub fn browse(&self, parent_id: &Uuid) -> Result<(Vec<SecretFolder>, Vec<LoginEntry>)> {
        self.store.browse(parent_id).map_err(Error::from)
    }

    pub fn list_folders(&self, parent_id: Uuid) -> Vec<SecretFolder> {
        self.store.folders(parent_id)
    }

    pub fn list_entries(&self, parent_id: Uuid) -> Vec<LoginEntry> {
        self.store.entries(parent_id)
    }

    pub fn list_entries_revealed(&self, parent_id: Uuid) -> Result<Vec<LoginEntryView>> {
        let key = self.secrets_key()?;
        let cipher = self.session.cipher();
        self.store
            .entries(parent_id)
            .into_iter()
            .map(|entry| entry.reveal(&key, cipher).map_err(Error::from))
            .collect()
    }

    pub fn add_folder(&mut self, parent_id: Uuid, name: String) -> Result<Uuid> {
        self.store.add_folder(parent_id, name).map_err(Error::from)
    }

    pub fn rename_folder(&mut self, id: Uuid, name: String) -> Result {
        self.store.rename_folder(id, name).map_err(Error::from)
    }

    pub fn remove_folder(&mut self, id: Uuid) -> Result {
        self.store.remove_folder(id).map_err(Error::from)
    }

    pub fn add_login(
        &mut self,
        folder_id: Uuid,
        name: String,
        username: String,
        password: String,
        website: Option<String>,
        comments: Option<String>,
        totp: Option<TOTP>,
    ) -> Result<Uuid> {
        let key = self.secrets_key()?;
        let entry = LoginEntry::seal(
            folder_id,
            name,
            username,
            password,
            website,
            comments,
            totp,
            &key,
            self.session.cipher(),
        )?;

        self.store.add_entry(entry).map_err(Error::from)
    }

    pub fn update_login(
        &mut self,
        id: Uuid,
        folder_id: Option<Uuid>,
        name: Option<String>,
        username: Option<String>,
        password: Option<String>,
        website: Option<String>,
        comments: Option<String>,
        totp: Option<Option<TOTP>>,
    ) -> Result {
        let key = self.secrets_key()?;
        let patch = LoginEntryPatch::from_plaintext(
            folder_id,
            name,
            username,
            password,
            website,
            comments,
            totp,
            &key,
            self.session.cipher(),
        )?;

        self.store.update_entry(id, patch).map_err(Error::from)
    }

    pub fn move_entry(&mut self, id: Uuid, new_folder_id: Uuid) -> Result {
        self.store.move_entry(id, new_folder_id).map_err(Error::from)
    }

    pub fn remove_entry(&mut self, id: Uuid) -> Result {
        self.store.remove_entry(id).map_err(Error::from)
    }

    pub fn entry(&self, id: &Uuid) -> Result<LoginEntry> {
        self.store
            .entry(id)
            .cloned()
            .ok_or_else(|| Error::ItemNotFound(id.to_string()))
    }

    pub fn reveal_entry(&self, id: &Uuid) -> Result<LoginEntryView> {
        let key = self.secrets_key()?;
        let cipher = self.session.cipher();
        let entry = self
            .store
            .entry(id)
            .ok_or_else(|| Error::ItemNotFound(id.to_string()))?;

        entry.reveal(&key, cipher).map_err(Error::from)
    }

    fn secrets_key(&self) -> Result<DerivedKey> {
        self.session
            .keyring()
            .derive_feature_key(self.session.version(), SECRETS_FEATURE_ID)
            .map_err(Error::from)
    }
}
