use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::file::File;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileSystemNode {
    Folder(Folder),
    File(File),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub children: Vec<FileSystemNode>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
