use serde::{Deserialize, Serialize};

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
    pub item_type: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameItemParams {
    pub id: String,
    pub item_type: String,
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
pub struct GetFileContentParams {
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderItem {
    pub id: String,
    pub name: String,
    pub item_count: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileItem {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub extension: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResult {
    pub folders: Vec<FolderItem>,
    pub files: Vec<FileItem>,
}
