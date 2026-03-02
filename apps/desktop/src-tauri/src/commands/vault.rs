use crate::commands::contracts::{CreateVaultParams, OpenVaultParams};
use crate::errors::{Error, Result};
use crate::state::TauriState;
use openvault_sdk::{CompressionAlgorithm, CreateConfig, EncryptionAlgorithm};
use std::path::PathBuf;
use std::str::FromStr;

#[tauri::command]
pub async fn create_vault(state: TauriState<'_>, params: CreateVaultParams) -> Result {
    let encryption = EncryptionAlgorithm::from_str(&params.encryption)
        .map_err(|e| Error::InvalidEncryption(e.to_string()))?;
    let compression = CompressionAlgorithm::from_str(&params.compression)
        .map_err(|e| Error::InvalidCompression(e.to_string()))?;

    let config = CreateConfig::default()
        .with_filename(params.name)
        .with_encryption(encryption)
        .with_compression(compression);

    let vault =
        openvault_sdk::create_and_open_vault(PathBuf::from(params.path), params.password, config)?;

    let mut vault_state = state.vault.lock().unwrap();
    *vault_state = Some(vault);

    Ok(())
}

#[tauri::command]
pub async fn open_vault(state: TauriState<'_>, params: OpenVaultParams) -> Result {
    let vault = openvault_sdk::open_vault(PathBuf::from(params.path), params.password)?;

    let mut vault_state = state.vault.lock().unwrap();
    *vault_state = Some(vault);

    Ok(())
}
