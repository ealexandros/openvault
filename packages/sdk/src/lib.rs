mod client;
mod errors;
mod features;
mod internal;
mod vault;

use std::path::Path;

use crate::client::VaultClient;
use crate::errors::Result;

pub use errors::Error;
pub use vault::Vault;

pub use openvault_core::features::filesystem::ROOT_FOLDER_ID;

pub use openvault_core::features::filesystem::{FileMetadata, FolderMetadata};
pub use openvault_core::features::messages::{MessageContact, MessageCredentials};
pub use openvault_core::operations::config::CreateConfig;
pub use openvault_crypto::compression::CompressionAlgorithm;
pub use openvault_crypto::encryption::EncryptionAlgorithm;

pub fn client() -> VaultClient {
    VaultClient::new()
}

pub fn create_vault(path: impl AsRef<Path>, password: impl AsRef<[u8]>) -> Result {
    client().create(path, password)
}

pub fn create_vault_with(
    path: impl AsRef<Path>,
    password: impl AsRef<[u8]>,
    options: CreateConfig,
) -> Result {
    client().create_with(path, password, options)
}

pub fn open_vault(path: impl AsRef<Path>, password: impl AsRef<[u8]>) -> Result<Vault> {
    client().open(path, password)
}

pub fn create_and_open_vault(
    path: impl AsRef<Path>,
    password: impl AsRef<[u8]>,
    options: CreateConfig,
) -> Result<Vault> {
    client().create_and_open(path, password, options)
}
