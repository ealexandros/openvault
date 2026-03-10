use std::fs::File;
use std::path::Path;
use uuid::Uuid;

use openvault_core::features::filesystem::{FileMetadata, FilesystemStore, FolderMetadata};
use openvault_core::internal::scanner::{ScannedFolder, scan_directory};
use openvault_core::operations::blob;
use openvault_core::repositories::{FeatureRepository, FilesystemRepository};
use openvault_core::vault::runtime::VaultSession;

use crate::errors::{Error, Result};
use crate::internal::file::{
    find_available_path, resolve_export_file_destination, resolve_export_folder_destination,
    resolve_export_root_destination,
};

pub struct FilesystemService<'a> {
    session: &'a mut VaultSession,
    store: &'a mut FilesystemStore,
}

impl<'a> FilesystemService<'a> {
    pub fn new(session: &'a mut VaultSession, store: &'a mut FilesystemStore) -> Self {
        FilesystemService { session, store }
    }

    pub fn read_file_bytes(&mut self, id: Uuid) -> Result<Vec<u8>> {
        let blob_ref = self.get_file(&id)?.blob.clone();

        blob::read_blob(self.session, &blob_ref).map_err(Error::from)
    }

    pub fn browse(&mut self, parent_id: &Uuid) -> Result<(Vec<FolderMetadata>, Vec<FileMetadata>)> {
        self.store.browse(parent_id).map_err(Error::from)
    }

    pub fn add_folder(&mut self, parent_id: Uuid, name: String) -> Result<Uuid> {
        self.store.add_folder(parent_id, name).map_err(Error::from)
    }

    pub fn add_file(&mut self, parent_id: Uuid, source_path: &Path) -> Result<Uuid> {
        let name = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or(Error::InvalidPath)?
            .to_owned();

        let extension = source_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default()
            .to_owned();

        let mut file = File::open(source_path)?;
        let blob_ref = blob::write_blob(self.session, &mut file)?;

        let file_id = self.store.add_file(parent_id, name, extension, blob_ref)?;

        Ok(file_id)
    }

    pub fn upload_folder(&mut self, parent_id: Uuid, source_path: &Path) -> Result<Uuid> {
        let scanned = scan_directory(source_path)?;
        self.upload_scanned_folder(parent_id, &scanned)
    }

    pub fn export_file(&mut self, id: Uuid, destination_path: &Path) -> Result {
        let file = self.get_file(&id)?;

        let destination = resolve_export_file_destination(destination_path, &file.name)?;

        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = self.read_file_bytes(id)?;
        std::fs::write(destination, content)?;

        Ok(())
    }

    pub fn export_folder(&mut self, id: Uuid, destination_path: &Path) -> Result {
        let folder = self.get_folder(&id)?;

        let destination = if folder.parent_id.is_none() {
            resolve_export_root_destination(destination_path)?
        } else {
            resolve_export_folder_destination(destination_path, &folder.name)?
        };

        std::fs::create_dir_all(&destination)?;
        self.export_folder_contents(id, &destination)
    }

    pub fn set_folder_icon(&mut self, id: Uuid, new_icon: String) -> Result {
        self.store
            .set_folder_icon(id, new_icon)
            .map_err(Error::from)
    }

    pub fn rename_folder(&mut self, id: Uuid, new_name: String) -> Result {
        self.store.rename_folder(id, new_name).map_err(Error::from)
    }

    pub fn rename_file(&mut self, id: Uuid, new_name: String) -> Result {
        self.store.rename_file(id, new_name).map_err(Error::from)
    }

    pub fn remove_folder(&mut self, id: Uuid) -> Result {
        self.store.remove_folder(id).map_err(Error::from)
    }

    pub fn remove_file(&mut self, id: Uuid) -> Result {
        self.store.remove_file(id).map_err(Error::from)
    }

    pub fn set_folder_favorite(&mut self, id: Uuid, is_favourite: bool) -> Result {
        self.store
            .set_folder_favorite(id, is_favourite)
            .map_err(Error::from)
    }

    pub fn set_file_favorite(&mut self, id: Uuid, is_favourite: bool) -> Result {
        self.store
            .set_file_favorite(id, is_favourite)
            .map_err(Error::from)
    }

    pub fn folder_children_count(&self, folder_id: &Uuid) -> usize {
        self.store.folder_children_count(folder_id)
    }

    pub fn folder_total_size_bytes(&self, folder_id: &Uuid) -> u64 {
        self.store.folder_total_size_bytes(folder_id)
    }

    pub fn reload(&mut self) -> Result<&FilesystemStore> {
        *self.store = FilesystemRepository::load(self.session)?;
        Ok(self.store)
    }

    fn get_file(&self, id: &Uuid) -> Result<&FileMetadata> {
        self.store
            .file(id)
            .ok_or(Error::ItemNotFound(id.to_string()))
    }

    fn get_folder(&self, id: &Uuid) -> Result<&FolderMetadata> {
        self.store
            .folder(id)
            .ok_or(Error::ItemNotFound(id.to_string()))
    }

    fn upload_scanned_folder(&mut self, parent_id: Uuid, folder: &ScannedFolder) -> Result<Uuid> {
        let folder_id = self.add_folder(parent_id, folder.name.clone())?;

        for file_path in &folder.files {
            self.add_file(folder_id, file_path)?;
        }
        for child in &folder.children {
            self.upload_scanned_folder(folder_id, child)?;
        }

        Ok(folder_id)
    }

    fn export_folder_contents(&mut self, folder_id: Uuid, destination_path: &Path) -> Result {
        let (folders, files) = self.store.browse(&folder_id)?;

        for file in files {
            let file_path = find_available_path(&destination_path.join(&file.name), true)?;
            let content = blob::read_blob(self.session, &file.blob)?;
            std::fs::write(file_path, content)?;
        }

        for folder in folders {
            let folder_path = find_available_path(&destination_path.join(&folder.name), false)?;
            std::fs::create_dir_all(&folder_path)?;
            self.export_folder_contents(folder.id, &folder_path)?;
        }

        Ok(())
    }
}
