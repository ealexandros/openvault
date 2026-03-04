use openvault_core::errors::Error;
use openvault_core::features::filesystem::scan_directory;

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn create_test_dir(name: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    dir.push(format!("scan_test_{}_{}", name, nanos));

    fs::create_dir(&dir).unwrap();
    dir
}

fn cleanup(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

#[test]
fn returns_error_if_not_directory() {
    let dir = create_test_dir("not_dir");
    let file_path = dir.join("file.txt");
    File::create(&file_path).unwrap();

    let result = scan_directory(&file_path);

    cleanup(&dir);

    assert!(matches!(result, Err(Error::InvalidPath)));
}

#[test]
fn scans_empty_directory() {
    let dir = create_test_dir("empty");

    let scanned = scan_directory(&dir).unwrap();

    assert!(scanned.files.is_empty());
    assert!(scanned.children.is_empty());

    cleanup(&dir);
}

#[test]
fn scans_files_in_root() {
    let dir = create_test_dir("files");

    let file_path = dir.join("file.txt");
    File::create(&file_path).unwrap();

    let scanned = scan_directory(&dir).unwrap();

    assert_eq!(scanned.files.len(), 1);
    assert_eq!(scanned.children.len(), 0);
    assert_eq!(scanned.files[0].file_name().unwrap(), "file.txt");

    cleanup(&dir);
}

#[test]
fn scans_nested_directories() {
    let dir = create_test_dir("nested");

    let child_dir = dir.join("child");
    fs::create_dir(&child_dir).unwrap();

    let nested_file = child_dir.join("nested.txt");
    File::create(&nested_file).unwrap();

    let scanned = scan_directory(&dir).unwrap();

    assert_eq!(scanned.children.len(), 1);
    assert_eq!(scanned.children[0].name, "child");
    assert_eq!(scanned.children[0].files.len(), 1);
    assert_eq!(
        scanned.children[0].files[0].file_name().unwrap(),
        "nested.txt"
    );

    cleanup(&dir);
}

#[test]
fn excludes_ds_store() {
    let dir = create_test_dir("excluded");

    let excluded = dir.join(".DS_Store");
    File::create(&excluded).unwrap();

    let scanned = scan_directory(&dir).unwrap();

    assert!(scanned.files.is_empty());

    cleanup(&dir);
}
