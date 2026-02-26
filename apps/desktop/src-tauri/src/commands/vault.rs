use crate::errors::Result;
use crate::state::TauriState;
use openvault_sdk::{CompressionAlgorithm, CreateConfig, EncryptionAlgorithm};
use std::path::PathBuf;

#[tauri::command]
pub async fn create_vault(
    state: TauriState<'_>,
    path: String,
    name: String,
    password: String,
) -> Result {
    let config = CreateConfig::default()
        .with_filename(name)
        .with_encryption(EncryptionAlgorithm::XChaCha20Poly1305)
        .with_compression(CompressionAlgorithm::Zstd);

    let vault = openvault_sdk::create_and_open_vault(PathBuf::from(path), password, config)?;

    let mut vault_state = state.vault.lock().unwrap();
    *vault_state = Some(vault);

    Ok(())
}

#[tauri::command]
pub async fn open_vault(state: TauriState<'_>, path: String, password: String) -> Result {
    let vault = openvault_sdk::open_vault(PathBuf::from(path), password)?;

    let mut vault_state = state.vault.lock().unwrap();
    *vault_state = Some(vault);

    Ok(())
}
