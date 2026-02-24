mod client;
mod config;
mod error;
mod session;

pub use client::VaultClient;
pub use config::CreateVaultOptions;
pub use error::{Error, Result};
pub use session::VaultHandle;

pub use openvault_core::features::blob_ref::BlobRef;
pub use openvault_core::vault::versions::shared::replay::{ReplayRecord, ReplayState};
pub use openvault_crypto::compression::CompressionAlgorithm;
pub use openvault_crypto::encryption::EncryptionAlgorithm;

pub fn client() -> VaultClient {
    VaultClient::new()
}

pub fn create_vault<P: AsRef<std::path::Path>, S: AsRef<[u8]>>(path: P, password: S) -> Result {
    client().create(path, password)
}

pub fn create_vault_with<P: AsRef<std::path::Path>, S: AsRef<[u8]>>(
    path: P,
    password: S,
    options: CreateVaultOptions,
) -> Result {
    client().create_with(path, password, options)
}

pub fn open_vault<P: AsRef<std::path::Path>, S: AsRef<[u8]>>(
    path: P,
    password: S,
) -> Result<VaultHandle> {
    client().open(path, password)
}

pub fn create_and_open_vault<P: AsRef<std::path::Path>, S: AsRef<[u8]>>(
    path: P,
    password: S,
    options: CreateVaultOptions,
) -> Result<VaultHandle> {
    client().create_and_open(path, password, options)
}
