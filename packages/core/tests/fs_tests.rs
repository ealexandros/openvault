use std::fs::{self, File};

use openvault_core::internal::fs::create_new_file;

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

    let file = create_new_file(&file_path);
    assert!(file.is_ok());
}

#[test]
fn create_file_exclusive_fails_if_exists() {
    let dir = setup_temp_dir("create_file_exists");
    let file_path = dir.join("output.vault");

    File::create(&file_path).unwrap();
    let result = create_new_file(&file_path);

    assert!(result.is_err());
}
