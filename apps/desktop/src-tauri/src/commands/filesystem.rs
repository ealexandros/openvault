use uuid::Uuid;

use crate::commands::responses::{BrowseResponse, FileItem, FolderItem};
use crate::errors::{Error, Result};
use crate::state::TauriState;

fn parse_uuid(id: &str) -> Result<Uuid> {
    Uuid::parse_str(id).map_err(|_| Error::InvalidUuid(id.to_string()))
}

#[tauri::command]
pub async fn browse_vault(state: TauriState<'_>, parent_id: String) -> Result<BrowseResponse> {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state.as_mut().ok_or(Error::VaultNotOpened)?;

    let parent_uuid = parse_uuid(&parent_id)?;

    let mut fs = vault.filesystem();

    let (folders, files) = fs.browse(&parent_uuid)?;

    let folders = folders
        .iter()
        .map(|folder| FolderItem {
            id: folder.id.to_string(),
            name: folder.name.clone(),
            item_count: vault.filesystem().children_count(&folder.id) as u64,
        })
        .collect();

    let files = files
        .iter()
        .map(|file| FileItem {
            id: file.id.to_string(),
            name: file.name.clone(),
            size: file.blob.size_bytes,
            extension: file.extension.clone(),
        })
        .collect();

    Ok(BrowseResponse { folders, files })
}

#[tauri::command]
pub async fn create_folder(
    state: TauriState<'_>,
    parent_id: String,
    name: String,
) -> Result<String> {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state.as_mut().ok_or(Error::VaultNotOpened)?;

    let parent_uuid = parse_uuid(&parent_id)?;
    let new_folder_id = vault.filesystem().add_folder(parent_uuid, name)?;

    vault.commit_all()?;

    Ok(new_folder_id.to_string())
}

#[tauri::command]
pub async fn delete_item(state: TauriState<'_>, id: String, item_type: String) -> Result {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state.as_mut().ok_or(Error::VaultNotOpened)?;

    let uuid = parse_uuid(&id)?;

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
    let vault = vault_state.as_mut().ok_or(Error::VaultNotOpened)?;

    let uuid = parse_uuid(&id)?;
    let mut fs = vault.filesystem();

    match item_type.as_str() {
        "file" => fs.rename_file(uuid, new_name)?,
        "folder" => fs.rename_folder(uuid, new_name)?,
        _ => return Err(Error::Internal("Invalid item type".into())),
    }

    fs.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn upload_file(state: TauriState<'_>, parent_id: String, source_path: String) -> Result {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state.as_mut().ok_or(Error::VaultNotOpened)?;

    let mut fs = vault.filesystem();

    let parent_uuid = parse_uuid(&parent_id)?;
    let source_path = std::path::PathBuf::from(source_path);

    fs.add_file(parent_uuid, &source_path)?;
    fs.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn get_file_content(state: TauriState<'_>, id: String) -> Result<Option<Vec<u8>>> {
    let mut vault_state = state.vault.lock().unwrap();
    let vault = vault_state.as_mut().ok_or(Error::VaultNotOpened)?;

    let mut fs = vault.filesystem();

    let uuid = parse_uuid(&id)?;
    let content = fs.read_file_content(uuid)?;

    Ok(content)
}
