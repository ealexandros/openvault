use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use openvault_sdk::{CreateVaultOptions, Error, VaultClient};

fn temp_vault_path(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("openvault_sdk_{name}_{timestamp}"));
    fs::create_dir_all(&dir).expect("create temp directory");
    dir.join("vault.ov")
}

#[test]
fn sdk_create_and_blob_roundtrip() {
    let path = temp_vault_path("blob_roundtrip");
    let root = path.parent().expect("temp root").to_path_buf();

    let client = VaultClient::new();
    client
        .create_with(&path, b"password", CreateVaultOptions::default())
        .expect("create vault");

    let mut handle = client.open(&path, b"password").expect("open vault");
    assert_eq!(handle.version(), 1);

    let blob_ref = handle.put_blob_bytes(b"sdk-blob").expect("put blob");
    let blob = handle.get_blob(&blob_ref).expect("get blob");
    assert_eq!(blob, b"sdk-blob");

    let replay = handle.replay_since_checkpoint().expect("replay state");
    assert!(replay.checkpoint.is_none());
    assert!(replay.records.is_empty());

    drop(handle);
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn sdk_open_wrong_password_returns_unlock_failed() {
    let path = temp_vault_path("unlock_failed");
    let root = path.parent().expect("temp root").to_path_buf();

    let client = VaultClient::new();
    client
        .create_with(&path, b"password", CreateVaultOptions::default())
        .expect("create vault");

    let result = client.open(&path, b"wrong-password");
    assert!(matches!(result, Err(Error::UnlockFailed)));

    fs::remove_dir_all(root).expect("cleanup");
}
