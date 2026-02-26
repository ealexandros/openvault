use serde::Serialize;

#[derive(Serialize)]
pub struct FolderItem {
    pub id: String,
    pub name: String,
    pub item_count: u64,
}

#[derive(Serialize)]
pub struct FileItem {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub extension: String,
}

#[derive(Serialize)]
pub struct BrowseResponse {
    pub folders: Vec<FolderItem>,
    pub files: Vec<FileItem>,
}
