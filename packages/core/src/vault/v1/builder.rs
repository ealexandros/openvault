use crate::constants::SALT_LEN;
use crate::vault::v1::schema::entries::{FileMeta, FileSystemMeta, FolderMeta};
use crate::vault::v1::schema::header::VaultHeader;
use crate::vault::v1::schema::vault::{Vault, VaultMeta};

pub struct VaultBuilder {
    salt: [u8; SALT_LEN],
    filesystem: FileSystemMeta,
}

impl VaultBuilder {
    pub fn new(salt: [u8; SALT_LEN]) -> Self {
        Self {
            salt,
            filesystem: FileSystemMeta::default(),
        }
    }

    pub fn add_files(mut self, entries: Vec<FileMeta>) -> Self {
        self.filesystem.files.extend(entries);
        self
    }

    pub fn add_folders(mut self, entries: Vec<FolderMeta>) -> Self {
        self.filesystem.folders.extend(entries);
        self
    }

    pub fn build(self) -> Vault {
        Vault {
            header: VaultHeader {
                salt: self.salt,
                ..Default::default()
            },
            metadata: VaultMeta {
                filesystem: self.filesystem,
                ..Default::default()
            },
        }
    }
}
