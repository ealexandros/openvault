use openvault_core::utils::fs::PathExt;
use std::fs::{self, File};

fn setup_temp_dir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!("vault_fs_test_{}", name));

    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }

    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn create_file_exclusive_ok() {
    let dir = setup_temp_dir("create_file_ok");
    let file_path = dir.join("output.vault");

    let file = file_path.create_new_file();
    assert!(file.is_ok());
}

#[test]
fn create_file_exclusive_fails_if_exists() {
    let dir = setup_temp_dir("create_file_exists");
    let file_path = dir.join("output.vault");

    File::create(&file_path).unwrap();
    let result = file_path.create_new_file();

    assert!(result.is_err());
}

#[test]
fn path_relative_to_ok() {
    use std::path::Path;
    let root = Path::new("/root/dir");
    let path = Path::new("/root/dir/sub/file.txt");

    let rel = path.relative_to(root).unwrap();
    assert_eq!(rel, "sub/file.txt");
}

#[test]
fn path_relative_to_err() {
    use std::path::Path;
    let root = Path::new("/root/dir");
    let path = Path::new("/other/dir/file.txt");

    let result = path.relative_to(root);
    assert!(result.is_err());
}

#[test]
fn test_ensure_file_exists() {
    let dir = setup_temp_dir("ensure_file");
    let file_path = dir.join("test.txt");
    let dir_path = dir.join("subdir");

    fs::create_dir(&dir_path).unwrap();
    File::create(&file_path).unwrap();

    assert!(file_path.ensure_file_exists().is_ok());
    assert!(dir_path.ensure_file_exists().is_err());
    assert!(dir.join("nonexistent").ensure_file_exists().is_err());
}
