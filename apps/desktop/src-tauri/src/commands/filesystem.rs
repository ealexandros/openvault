use crate::errors::{Error, Result};
use crate::models::FilesystemItem;
use crate::state::TauriState;
use uuid::Uuid;

#[tauri::command]
pub async fn browse_vault(
    state: TauriState<'_>,
    parent_id: Option<String>,
) -> Result<Vec<FilesystemItem>> {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| Error::Internal("Vault not opened".into()))?;

    let parent_uuid = if let Some(id_str) = parent_id {
        Uuid::parse_str(&id_str).map_err(|_| Error::Internal("Invalid UUID".into()))?
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
            details: Some(vault.filesystem().children_count(&folder.id).to_string()),
            mime_type: None,
        });
    }

    for file in files {
        items.push(FilesystemItem {
            id: file.id.to_string(),
            name: file.name,
            item_type: "file".to_string(),
            details: Some(file.blob.size_bytes.to_string()),
            mime_type: Some(file.extension),
        });
    }

    Ok(items)
}

#[tauri::command]
pub async fn create_folder(
    state: TauriState<'_>,
    parent_id: Option<String>,
    name: String,
) -> Result<String> {
    let parent_id = parent_id.unwrap_or(Uuid::nil().to_string());

    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| Error::Internal("Vault not opened".into()))?;

    let parent_uuid =
        Uuid::parse_str(&parent_id).map_err(|_| Error::Internal("Invalid UUID".into()))?;

    let new_id = vault.filesystem().add_folder(parent_uuid, name)?;
    vault.commit_all()?;

    Ok(new_id.to_string())
}

#[tauri::command]
pub async fn delete_item(state: TauriState<'_>, id: String, item_type: String) -> Result {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| Error::Internal("Vault not opened".into()))?;

    let uuid = Uuid::parse_str(&id).map_err(|_| Error::Internal("Invalid UUID".into()))?;

    match item_type.as_str() {
        "file" => vault.filesystem().remove_file(uuid)?,
        "folder" => vault.filesystem().remove_folder(uuid)?,
        _ => return Err(Error::Internal("Invalid item type".into())),
    }

    vault.commit_all()?;
    Ok(())
}

#[tauri::command]
pub async fn rename_item(
    state: TauriState<'_>,
    id: String,
    item_type: String,
    new_name: String,
) -> Result {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| Error::Internal("Vault not opened".into()))?;

    let uuid = Uuid::parse_str(&id).map_err(|_| Error::Internal("Invalid UUID".into()))?;

    match item_type.as_str() {
        "file" => vault.filesystem().rename_file(uuid, new_name)?,
        "folder" => vault.filesystem().rename_folder(uuid, new_name)?,
        _ => return Err(Error::Internal("Invalid item type".into())),
    }

    vault.commit_all()?;
    Ok(())
}

#[tauri::command]
pub async fn upload_file(
    state: TauriState<'_>,
    parent_id: Option<String>,
    source_path: String,
) -> Result {
    let parent_id = parent_id.unwrap_or(Uuid::nil().to_string());

    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| Error::Internal("Vault not opened".into()))?;

    let parent_uuid =
        Uuid::parse_str(&parent_id).map_err(|_| Error::Internal("Invalid UUID".into()))?;

    let source_path = std::path::PathBuf::from(source_path);

    vault.filesystem().add_file(parent_uuid, &source_path)?;
    vault.commit_all()?;

    Ok(())
}

#[tauri::command]
pub async fn get_file_content(state: TauriState<'_>, id: String) -> Result<Option<Vec<u8>>> {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state
        .as_mut()
        .ok_or_else(|| Error::Internal("Vault not opened".into()))?;

    let uuid = Uuid::parse_str(&id).map_err(|_| Error::Internal("Invalid UUID".into()))?;

    let content = vault.filesystem().read_file_content(uuid)?;

    Ok(content)
}
