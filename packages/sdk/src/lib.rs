mod client;
mod config;
mod error;
mod features;
mod session;
mod stores;

use std::path::Path;

pub use client::VaultClient;
pub use config::CreateVaultOptions;
pub use error::{Error, Result};
pub use features::{FeatureFacade, FilesystemFeature, SecretsFeature};
pub use session::VaultHandle;
pub use stores::{StoresCommitResult, VaultStores};

pub use openvault_core::features::filesystem::{
    FileMetadata, FileMetadataPatch, FilesystemChange, FilesystemDelta, FilesystemError,
    FilesystemSnapshot, FilesystemStore, FolderMetadata, FolderMetadataPatch, ROOT_FOLDER_ID,
    scan_directory, scan_file,
};
pub use openvault_core::features::secrets::{
    EncryptedField, LoginEntry, LoginEntryPatch, SecretDelta, SecretError, SecretSnapshot,
    SecretStore, SecretsChange, TOTP,
};
pub use openvault_core::features::shared::blob_ref::BlobRef;
pub use openvault_core::vault::versions::shared::replay::{ReplayRecord, ReplayState};
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
    options: CreateVaultOptions,
) -> Result {
    client().create_with(path, password, options)
}

pub fn open_vault(path: impl AsRef<Path>, password: impl AsRef<[u8]>) -> Result<VaultHandle> {
    client().open(path, password)
}

pub fn create_and_open_vault(
    path: impl AsRef<Path>,
    password: impl AsRef<[u8]>,
    options: CreateVaultOptions,
) -> Result<VaultHandle> {
    client().create_and_open(path, password, options)
}
