use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    File,
    Folder,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseVaultParams {
    pub parent_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderParams {
    pub parent_id: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteItemParams {
    pub id: String,
    pub item_type: ItemType,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameItemParams {
    pub id: String,
    pub item_type: ItemType,
    pub new_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileParams {
    pub parent_id: String,
    pub source_path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFolderParams {
    pub parent_id: String,
    pub source_path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadFileParams {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathIsFileParams {
    pub path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultParams {
    pub path: String,
    pub name: String,
    pub password: String,
    pub encryption: String,
    pub compression: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenVaultParams {
    pub path: String,
    pub password: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFolderIconParams {
    pub id: String,
    pub icon: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFavoriteItemParams {
    pub id: String,
    pub is_favourite: bool,
    pub item_type: ItemType,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderItem {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub is_favourite: bool,
    pub item_count: u64,
    pub total_size_bytes: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileItem {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub is_favourite: bool,
    pub extension: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResult {
    pub folders: Vec<FolderItem>,
    pub files: Vec<FileItem>,
}
