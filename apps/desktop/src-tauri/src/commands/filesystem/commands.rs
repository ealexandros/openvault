use openvault_sdk::{FILESYSTEM_ROOT_FOLDER_ID, SecretVec};
use uuid::Uuid;

use super::contracts::{
    BrowseResult, BrowseVaultParams, ChangeFolderIconParams, CreateFolderParams, DeleteItemParams,
    ExportFileParams, ExportFolderParams, FileItem, FolderItem, ItemType, PathIsFileParams,
    ReadFileParams, RenameItemParams, SetFavoriteItemParams, UploadFileParams, UploadFolderParams,
};
use crate::errors::{Error, Result};
use crate::internal::parser::{parse_optional_uuid, parse_uuid};
use crate::protocols::secure;
use crate::state::TauriState;

macro_rules! vault_fs {
    ($state:expr, $fs:ident, $vault:ident) => {
        let mut $vault = $state.vault.lock().map_err(|_| Error::LockPoisoned)?;
        let $vault = $vault.as_mut().ok_or(Error::VaultNotOpened)?;
        let mut $fs = $vault.filesystem();
    };
}

#[tauri::command]
pub async fn path_is_file(params: PathIsFileParams) -> Result<bool> {
    Ok(std::path::Path::new(&params.path).is_file())
}

#[tauri::command]
pub async fn browse_fs(state: TauriState<'_>, params: BrowseVaultParams) -> Result<BrowseResult> {
    vault_fs!(state, fs, vault);

    let parent_uuid =
        parse_optional_uuid(params.parent_id.as_deref())?.unwrap_or(FILESYSTEM_ROOT_FOLDER_ID);

    let (folders, files) = fs.browse(&parent_uuid)?;

    let folders = folders
        .iter()
        .map(|folder| FolderItem {
            id: folder.id.to_string(),
            name: folder.name.clone(),
            icon: folder.icon.clone(),
            is_favourite: folder.is_favourite,
            item_count: fs.folder_children_count(&folder.id) as u64,
            total_size_bytes: fs.folder_total_size_bytes(&folder.id),
            created_at: folder.created_at.to_string(),
            updated_at: folder.updated_at.to_string(),
        })
        .collect();

    let files = files
        .iter()
        .map(|file| FileItem {
            id: file.id.to_string(),
            name: file.name.clone(),
            size: file.blob.size_bytes,
            is_favourite: file.is_favourite,
            extension: file.extension.clone(),
            created_at: file.created_at.to_string(),
            updated_at: file.updated_at.to_string(),
        })
        .collect();

    Ok(BrowseResult { folders, files })
}

#[tauri::command]
pub async fn create_folder(state: TauriState<'_>, params: CreateFolderParams) -> Result<String> {
    vault_fs!(state, fs, vault);

    let parent_uuid =
        parse_optional_uuid(params.parent_id.as_deref())?.unwrap_or(FILESYSTEM_ROOT_FOLDER_ID);

    let new_folder_id = fs.add_folder(parent_uuid, params.name)?;

    vault.commit()?;

    Ok(new_folder_id.to_string())
}

#[tauri::command]
pub async fn delete_item(state: TauriState<'_>, params: DeleteItemParams) -> Result {
    vault_fs!(state, fs, vault);

    let uuid = parse_uuid(&params.id)?;

    match params.item_type {
        ItemType::File => fs.remove_file(uuid)?,
        ItemType::Folder => fs.remove_folder(uuid)?,
    }

    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn rename_item(state: TauriState<'_>, params: RenameItemParams) -> Result {
    vault_fs!(state, fs, vault);

    let uuid = parse_uuid(&params.id)?;

    match params.item_type {
        ItemType::File => fs.rename_file(uuid, params.new_name)?,
        ItemType::Folder => fs.rename_folder(uuid, params.new_name)?,
    }

    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn upload_file(state: TauriState<'_>, params: UploadFileParams) -> Result {
    vault_fs!(state, fs, vault);

    let parent_uuid =
        parse_optional_uuid(params.parent_id.as_deref())?.unwrap_or(FILESYSTEM_ROOT_FOLDER_ID);

    let source_path = std::path::PathBuf::from(params.source_path);

    fs.add_file(parent_uuid, &source_path)?;
    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn upload_folder(state: TauriState<'_>, params: UploadFolderParams) -> Result {
    vault_fs!(state, fs, vault);

    let parent_uuid =
        parse_optional_uuid(params.parent_id.as_deref())?.unwrap_or(FILESYSTEM_ROOT_FOLDER_ID);

    let source_path = std::path::PathBuf::from(params.source_path);

    fs.upload_folder(parent_uuid, &source_path)?;
    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn expose_file_url(state: TauriState<'_>, params: ReadFileParams) -> Result<String> {
    vault_fs!(state, fs, vault);

    let uuid = parse_uuid(&params.id)?;
    let content = fs.read_file_bytes(uuid)?;

    let mut inner_map = state
        .secure_proto
        .lock()
        .map_err(|_| Error::Internal("Lock poisoned".to_string()))?;

    let token = Uuid::new_v4().to_string();
    let secret_vec = SecretVec::new(content)
        .map_err(|_| Error::Internal("Failed to create secret vec".to_string()))?;

    inner_map.insert(token.clone(), secret_vec);

    Ok(secure::protocol_uri(&token))
}

#[tauri::command]
pub async fn set_folder_icon(state: TauriState<'_>, params: ChangeFolderIconParams) -> Result {
    vault_fs!(state, fs, vault);

    let uuid = parse_uuid(&params.id)?;
    fs.set_folder_icon(uuid, params.icon)?;
    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn set_favorite_item(state: TauriState<'_>, params: SetFavoriteItemParams) -> Result {
    vault_fs!(state, fs, vault);

    let uuid = parse_uuid(&params.id)?;

    match params.item_type {
        ItemType::File => fs.set_file_favorite(uuid, params.is_favourite)?,
        ItemType::Folder => fs.set_folder_favorite(uuid, params.is_favourite)?,
    }

    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn export_file(state: TauriState<'_>, params: ExportFileParams) -> Result {
    vault_fs!(state, fs, vault);

    let uuid = parse_uuid(&params.id)?;
    let destination_path = std::path::PathBuf::from(params.destination_path);

    fs.export_file(uuid, &destination_path)?;

    Ok(())
}

#[tauri::command]
pub async fn export_folder(state: TauriState<'_>, params: ExportFolderParams) -> Result {
    vault_fs!(state, fs, vault);

    let uuid = parse_uuid(&params.id)?;
    let destination_path = std::path::PathBuf::from(params.destination_path);

    fs.export_folder(uuid, &destination_path)?;

    Ok(())
}
