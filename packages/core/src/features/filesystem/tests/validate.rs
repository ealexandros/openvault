use crate::features::filesystem::errors::FilesystemError;
use crate::features::filesystem::models::{FileMetadata, FolderMetadata};
use crate::features::filesystem::validate::{
    validate_file_name, validate_folder_name, validate_no_cycle, validate_safe_name,
    validate_snapshot,
};
use crate::features::shared::BlobRef;
use std::collections::HashMap;
use uuid::Uuid;

#[test]
fn test_validate_no_cycle() {
    let mut folders = HashMap::new();
    let root = FolderMetadata::root();
    let root_id = root.id;
    folders.insert(root_id, root);

    let folder1 = FolderMetadata::new(Some(root_id), "folder1".to_string());
    let folder1_id = folder1.id;
    folders.insert(folder1_id, folder1.clone());

    let folder2 = FolderMetadata::new(Some(folder1_id), "folder2".to_string());
    let folder2_id = folder2.id;
    folders.insert(folder2_id, folder2.clone());

    assert!(validate_no_cycle(&folders, folder2_id, root_id).is_ok());

    match validate_no_cycle(&folders, folder1_id, folder2_id) {
        Err(FilesystemError::CycleDetected(id)) => assert_eq!(id, folder1_id),
        res => panic!("Expected CycleDetected error, got {:?}", res),
    }

    match validate_no_cycle(&folders, folder1_id, folder1_id) {
        Err(FilesystemError::CycleDetected(id)) => assert_eq!(id, folder1_id),
        res => panic!("Expected CycleDetected error, got {:?}", res),
    }
}

#[test]
fn test_validate_folder_name() {
    let mut folders = HashMap::new();
    let root = FolderMetadata::root();
    let root_id = root.id;
    folders.insert(root_id, root);

    let folder1 = FolderMetadata::new(Some(root_id), "my_folder".to_string());
    folders.insert(folder1.id, folder1.clone());

    assert!(validate_folder_name(root_id, "my_folder", &folders).is_err());

    assert!(validate_folder_name(root_id, "other_folder", &folders).is_ok());
}

#[test]
fn test_validate_file_name() {
    let mut files = HashMap::new();
    let root_id = Uuid::new_v4();

    let blob = BlobRef {
        id: Uuid::new_v4(),
        size_bytes: 100,
        manifest_offset: 0,
    };
    let file1 = FileMetadata::new(root_id, "report".to_string(), "txt".to_string(), blob);
    files.insert(file1.id, file1.clone());

    assert!(validate_file_name(root_id, "report", &files).is_err());

    assert!(validate_file_name(root_id, "report_new", &files).is_ok());
}

#[test]
fn test_validate_safe_name() {
    assert!(validate_safe_name("valid_name").is_ok());
    assert!(validate_safe_name("valid name").is_ok());

    assert!(validate_safe_name(".").is_err());
    assert!(validate_safe_name("..").is_err());
    assert!(validate_safe_name(" invalid").is_err());
    assert!(validate_safe_name("invalid ").is_err());
    assert!(validate_safe_name("invalid\n").is_err());
}

#[test]
fn test_validate_snapshot() {
    let mut folders = HashMap::new();
    let files = HashMap::new();

    assert!(validate_snapshot(&folders, &files).is_err());

    let root = FolderMetadata::root();
    let root_id = root.id;
    folders.insert(root_id, root);

    assert!(validate_snapshot(&folders, &files).is_ok());

    let folder1 = FolderMetadata::new(Some(root_id), "f1".to_string());
    folders.insert(folder1.id, folder1.clone());

    assert!(validate_snapshot(&folders, &files).is_ok());

    let folder2 = FolderMetadata::new(Some(root_id), "f1".to_string());
    folders.insert(folder2.id, folder2.clone());

    assert!(validate_snapshot(&folders, &files).is_err());
}
