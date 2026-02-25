use std::path::Path;

use openvault_core::operations::config::CreateConfig;
use openvault_core::operations::vault::{
    create_and_open_vault, create_vault, create_vault_with, open_vault,
};

use crate::errors::Error;
use crate::errors::Result;
use crate::vault::Vault;

#[derive(Debug, Default, Clone, Copy)]
pub struct VaultClient;

impl VaultClient {
    pub fn new() -> Self {
        Self
    }

    pub fn create(&self, path: impl AsRef<Path>, password: impl AsRef<[u8]>) -> Result {
        create_vault(path.as_ref(), password.as_ref()).map_err(Into::into)
    }

    pub fn create_with(
        &self,
        path: impl AsRef<Path>,
        password: impl AsRef<[u8]>,
        options: CreateConfig,
    ) -> Result {
        create_vault_with(path.as_ref(), password.as_ref(), options).map_err(Into::into)
    }

    pub fn open(&self, path: impl AsRef<Path>, password: impl AsRef<[u8]>) -> Result<Vault> {
        let session = open_vault(path.as_ref(), password.as_ref()).map_err(Error::from)?;

        Vault::new(session)
    }

    pub fn create_and_open(
        &self,
        path: impl AsRef<Path>,
        password: impl AsRef<[u8]>,
        options: CreateConfig,
    ) -> Result<Vault> {
        let session = create_and_open_vault(path.as_ref(), password.as_ref(), options)
            .map_err(Error::from)?;

        Vault::new(session)
    }
}
