use openvault_sdk::{CompressionAlgorithm, CreateConfig, EncryptionAlgorithm};
use std::path::PathBuf;
use std::str::FromStr;
use zeroize::Zeroize;

use super::contracts::{CreateVaultParams, OpenVaultParams};
use crate::commands::vault::contracts::CreateVaultResult;
use crate::errors::{Error, Result};
use crate::internal::format::string_from_bytes;
use crate::state::TauriState;

#[tauri::command]
pub async fn create_vault(
    state: TauriState<'_>,
    params: CreateVaultParams,
) -> Result<CreateVaultResult> {
    let encryption = EncryptionAlgorithm::from_str(&params.encryption)
        .map_err(|e| Error::InvalidEncryption(e.to_string()))?;
    let compression = CompressionAlgorithm::from_str(&params.compression)
        .map_err(|e| Error::InvalidCompression(e.to_string()))?;

    let config = CreateConfig::default()
        .with_filename(params.name)
        .with_encryption(encryption)
        .with_compression(compression);

    let mut password = string_from_bytes(params.password)?;

    let vault =
        openvault_sdk::create_and_open_vault(PathBuf::from(params.path), &password, config)?;
    password.zeroize();

    let path = vault.path().to_path_buf();

    let mut vault_state = state.vault.lock().map_err(|_| Error::LockPoisoned)?;
    *vault_state = Some(vault);

    Ok(CreateVaultResult {
        path: path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
pub async fn open_vault(state: TauriState<'_>, params: OpenVaultParams) -> Result {
    let path = PathBuf::from(params.path);
    let mut password = string_from_bytes(params.password)?;

    let vault = openvault_sdk::open_vault(path, &password)?;

    password.zeroize();

    let mut vault_state = state.vault.lock().map_err(|_| Error::LockPoisoned)?;
    *vault_state = Some(vault);

    Ok(())
}

#[tauri::command]
pub async fn lock_vault(state: TauriState<'_>) -> Result {
    let mut vault_lock = state.vault.lock().map_err(|_| Error::LockPoisoned)?;

    if let Some(mut vault) = vault_lock.take() {
        vault.zeroize();
    }

    Ok(())
}

#[tauri::command]
pub async fn compact_vault(state: TauriState<'_>) -> Result {
    let mut vault_state = state.vault.lock().map_err(|_| Error::LockPoisoned)?;
    let vault = vault_state.as_mut().ok_or(Error::VaultNotOpened)?;

    vault.compact()?;

    Ok(())
}
