mod errors;

use openvault_sdk::{CompressionAlgorithm, CreateConfig, EncryptionAlgorithm, Vault};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

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

#[derive(serde::Serialize)]
struct FilesystemItem {
    id: String,
    name: String,
    #[serde(rename = "type")]
    item_type: String, // "file" | "folder"
    details: Option<String>,
}

#[tauri::command]
async fn open_vault(state: TauriState<'_>, path: String, password: String) -> Result {
    let vault = openvault_sdk::open_vault(PathBuf::from(path), password)?;

    let mut vault_state = state.vault.lock().unwrap();
    *vault_state = Some(vault);

    Ok(())
}

#[tauri::command]
async fn browse_vault(
    state: TauriState<'_>,
    parent_id: Option<String>,
) -> Result<Vec<FilesystemItem>> {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| crate::errors::Error::Internal("Vault not opened".into()))?;

    let parent_uuid = if let Some(id_str) = parent_id {
        uuid::Uuid::parse_str(&id_str)
            .map_err(|_| crate::errors::Error::Internal("Invalid UUID".into()))?
    } else {
        openvault_sdk::ROOT_FOLDER_ID
    };

    let (folders, files) = vault.filesystem().browse(&parent_uuid)?;

    let mut items = Vec::new();

    for folder in folders {
        items.push(FilesystemItem {
            id: folder.id.to_string(),
            name: folder.name,
            item_type: "folder".to_string(),
            details: Some(vault.filesystem().count_children(&folder.id).to_string()),
        });
    }

    for file in files {
        items.push(FilesystemItem {
            id: file.id.to_string(),
            name: file.name,
            item_type: "file".to_string(),
            details: Some(file.blob.size_bytes.to_string()),
        });
    }

    Ok(items)
}

#[tauri::command]
async fn create_folder(
    state: TauriState<'_>,
    parent_id: Option<String>,
    name: String,
) -> Result<String> {
    let parent_id = parent_id.unwrap_or(Uuid::nil().to_string());

    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| crate::errors::Error::Internal("Vault not opened".into()))?;

    let parent_uuid = uuid::Uuid::parse_str(&parent_id)
        .map_err(|_| crate::errors::Error::Internal("Invalid UUID".into()))?;

    let new_id = vault.filesystem().add_folder(parent_uuid, name)?;
    vault.commit_all()?;

    Ok(new_id.to_string())
}

#[tauri::command]
async fn delete_item(state: TauriState<'_>, id: String, item_type: String) -> Result {
    let mut vault_state = state.vault.lock().unwrap();

    let vault = vault_state
        .as_mut()
        .ok_or_else(|| crate::errors::Error::Internal("Vault not opened".into()))?;

    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| crate::errors::Error::Internal("Invalid UUID".into()))?;

    match item_type.as_str() {
        "file" => vault.filesystem().delete_file(uuid)?,
        "folder" => vault.filesystem().delete_folder(uuid)?,
        _ => return Err(crate::errors::Error::Internal("Invalid item type".into())),
    }

    vault.commit_all()?;

    Ok(())
}

#[tauri::command]
async fn rename_item(
    state: TauriState<'_>,
    id: String,
    item_type: String,
    new_name: String,
) -> Result {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| crate::errors::Error::Internal("Vault not opened".into()))?;

    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| crate::errors::Error::Internal("Invalid UUID".into()))?;

    match item_type.as_str() {
        "file" => vault.filesystem().rename_file(uuid, new_name)?,
        "folder" => vault.filesystem().rename_folder(uuid, new_name)?,
        _ => return Err(crate::errors::Error::Internal("Invalid item type".into())),
    }

    vault.commit_all()?;

    Ok(())
}

#[tauri::command]
async fn upload_file(
    state: TauriState<'_>,
    parent_id: Option<String>,
    source_path: String,
) -> Result {
    let parent_id = parent_id.unwrap_or(Uuid::nil().to_string());

    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| crate::errors::Error::Internal("Vault not opened".into()))?;

    let parent_uuid = uuid::Uuid::parse_str(&parent_id)
        .map_err(|_| crate::errors::Error::Internal("Invalid UUID".into()))?;

    let source_path = std::path::PathBuf::from(source_path);

    vault.filesystem().add_file(parent_uuid, &source_path)?;
    vault.commit_all()?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            create_vault,
            open_vault,
            browse_vault,
            create_folder,
            delete_item,
            rename_item,
            upload_file
        ])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
