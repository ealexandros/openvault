use std::path::Path;

use openvault_core::operations::vault::{
    create_and_open_vault, create_vault, create_vault_with, open_vault,
};

use crate::config::CreateVaultOptions;
use crate::error::Error;
use crate::error::Result;
use crate::session::VaultHandle;

#[derive(Debug, Default, Clone, Copy)]
pub struct VaultClient;

impl VaultClient {
    pub fn new() -> Self {
        Self
    }

    pub fn create<P: AsRef<Path>, S: AsRef<[u8]>>(&self, path: P, password: S) -> Result {
        create_vault(path.as_ref(), password.as_ref()).map_err(Into::into)
    }

    pub fn create_with<P: AsRef<Path>, S: AsRef<[u8]>>(
        &self,
        path: P,
        password: S,
        options: CreateVaultOptions,
    ) -> Result {
        create_vault_with(path.as_ref(), password.as_ref(), options.into_core()).map_err(Into::into)
    }

    pub fn open<P: AsRef<Path>, S: AsRef<[u8]>>(
        &self,
        path: P,
        password: S,
    ) -> Result<VaultHandle> {
        let session = open_vault(path.as_ref(), password.as_ref()).map_err(Error::from)?;
        Ok(VaultHandle::new(session))
    }

    pub fn create_and_open<P: AsRef<Path>, S: AsRef<[u8]>>(
        &self,
        path: P,
        password: S,
        options: CreateVaultOptions,
    ) -> Result<VaultHandle> {
        let session = create_and_open_vault(path.as_ref(), password.as_ref(), options.into_core())
            .map_err(Error::from)?;

        Ok(VaultHandle::new(session))
    }
}
