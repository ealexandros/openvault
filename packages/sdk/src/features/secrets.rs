use uuid::Uuid;

use openvault_core::features::secrets::{
    EncryptedField, LoginEntry, LoginEntryPatch, SecretDelta, SecretStore,
};
use openvault_core::operations::secrets::{commit_secret_store, load_secret_store};
use openvault_core::vault::runtime::VaultSession;

use crate::errors::{Error, Result};

pub struct SecretsFeature<'a> {
    session: &'a mut VaultSession,
    state: Option<SecretStore>,
}

impl<'a> SecretsFeature<'a> {
    pub(crate) fn new(session: &'a mut VaultSession) -> Self {
        Self {
            session,
            state: None,
        }
    }

    pub fn load(&mut self) -> Result<&mut SecretStore> {
        if self.state.is_none() {
            self.state = Some(load_secret_store(self.session)?);
        }

        Ok(self.state.as_mut().expect("state loaded"))
    }

    pub fn refresh(&mut self) -> Result<&mut SecretStore> {
        self.state = Some(load_secret_store(self.session)?);
        Ok(self.state.as_mut().expect("state refreshed"))
    }

    pub fn clear(&mut self) {
        self.state = None;
    }

    pub fn commit(&mut self) -> Result<bool> {
        let Some(state) = self.state.as_mut() else {
            return Ok(false);
        };

        commit_secret_store(self.session, state).map_err(Into::into)
    }

    pub fn add_delta(&mut self, delta: SecretDelta) -> Result {
        let store = self.load()?;

        match delta {
            SecretDelta::Added(entry) => store.insert(entry).map(|_| ()).map_err(map_secret_error),
            SecretDelta::Updated { id, patch } => store.update(id, patch).map_err(map_secret_error),
            SecretDelta::Deleted { id } => store.delete(id).map_err(map_secret_error),
        }
    }

    pub fn insert(&mut self, entry: LoginEntry) -> Result<Uuid> {
        let id = entry.id;
        self.add_delta(SecretDelta::Added(entry))?;
        Ok(id)
    }

    pub fn update(&mut self, id: Uuid, patch: LoginEntryPatch) -> Result {
        self.add_delta(SecretDelta::Updated { id, patch })
    }

    pub fn delete(&mut self, id: Uuid) -> Result {
        self.add_delta(SecretDelta::Deleted { id })
    }

    pub fn list_all(&mut self) -> Result<Vec<LoginEntry>> {
        Ok(self.load()?.list_all())
    }

    pub fn list_by_folder(&mut self, folder: &str) -> Result<Vec<LoginEntry>> {
        Ok(self.load()?.list_by_folder(folder))
    }

    pub fn show_password(&mut self, id: &Uuid) -> Result<Option<EncryptedField>> {
        Ok(self.load()?.show_password(id))
    }

    pub fn get_entry(&mut self, id: &Uuid) -> Result<Option<LoginEntry>> {
        Ok(self.load()?.get_entry(id))
    }
}

fn map_secret_error(error: openvault_core::features::secrets::SecretError) -> Error {
    Error::from(openvault_core::errors::Error::from(error))
}
