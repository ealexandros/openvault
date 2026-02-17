use chrono::{DateTime, Utc};
use uuid::Uuid;

// @todo-now rethink about this & add soft delete

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    // pub content_type: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileSystem {
    pub files: Vec<File>,
    pub folders: Vec<Folder>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl File {
    pub fn new(
        id: Uuid,
        parent_id: Option<Uuid>,
        name: String,
        // content_type: String,
        size: u64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            parent_id,
            // content_type,
            size,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Folder {
    pub fn new(id: Uuid, parent_id: Option<Uuid>, name: String) -> Self {
        let now = Utc::now();

        Self {
            id,
            name,
            parent_id,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn root() -> Self {
        Self::new(Uuid::nil(), None, "/".to_string())
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            folders: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
    }

    pub fn add_files(&mut self, files: Vec<File>) -> &mut Self {
        self.files.extend(files);
        self
    }

    pub fn add_folders(&mut self, folders: Vec<Folder>) -> &mut Self {
        self.folders.extend(folders);
        self
    }
}
