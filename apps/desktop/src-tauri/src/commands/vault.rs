use crate::commands::contracts::{CreateVaultParams, OpenVaultParams};
use crate::errors::{Error, Result};
use crate::state::TauriState;
use openvault_sdk::{CompressionAlgorithm, CreateConfig, EncryptionAlgorithm};
use std::path::PathBuf;
use std::str::FromStr;
use zeroize::Zeroize;

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

    let mut password = params.password;
    let vault =
        openvault_sdk::create_and_open_vault(PathBuf::from(params.path), password.clone(), config)?;
    password.zeroize();

    let mut vault_state = state.vault.lock().unwrap();
    *vault_state = Some(vault);

    Ok(())
}

#[tauri::command]
pub async fn open_vault(state: TauriState<'_>, params: OpenVaultParams) -> Result {
    let path = PathBuf::from(params.path);
    let mut password = String::from_utf8(params.password).map_err(|_| Error::InvalidUtf8)?;

    let vault = openvault_sdk::open_vault(path, password.clone())?;

    password.zeroize();

    let mut vault_state = state.vault.lock().unwrap();
    *vault_state = Some(vault);

    Ok(())
}

#[tauri::command]
pub async fn lock_vault(state: TauriState<'_>) -> Result {
    if let Some(mut vault) = state.vault.lock().unwrap().take() {
        vault.zeroize();
    }

    Ok(())
}

// @todo-now implement those

#[tauri::command]
pub async fn compact_vault(_state: TauriState<'_>) -> Result {
    todo!()
}
