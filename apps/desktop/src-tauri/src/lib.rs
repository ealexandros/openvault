mod errors;

use openvault_sdk::{CompressionAlgorithm, CreateConfig, EncryptionAlgorithm, Vault};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

use crate::errors::Result;

#[derive(Default)]
struct AppState {
    vault: Mutex<Option<Vault>>,
}

type TauriState<'a> = State<'a, AppState>;

#[tauri::command]
async fn create_vault(
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
async fn open_vault(state: TauriState<'_>, path: String, password: String) -> Result {
    let vault = openvault_sdk::open_vault(PathBuf::from(path), password)?;

    let mut vault_state = state.vault.lock().unwrap();
    *vault_state = Some(vault);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![create_vault, open_vault])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
