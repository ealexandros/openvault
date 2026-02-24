use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use openvault_sdk::{
    CreateVaultOptions, EncryptedField, Error, LoginEntry, VaultClient, scan_file,
};

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

#[test]
fn sdk_filesystem_store_persists_changes() {
    let path = temp_vault_path("filesystem_feature");
    let root = path.parent().expect("temp root").to_path_buf();

    let client = VaultClient::new();
    client
        .create_with(&path, b"password", CreateVaultOptions::default())
        .expect("create vault");

    {
        let mut handle = client.open(&path, b"password").expect("open vault");
        let mut store = handle
            .load_filesystem_store()
            .expect("load filesystem store");

        let file = scan_file(Path::new("readme.md")).expect("scan file");
        store.add_file(file).expect("add file");

        let wrote = handle
            .commit_filesystem_store(&mut store)
            .expect("commit filesystem store");
        assert!(wrote);
    }

    {
        let mut reopened = client.open(&path, b"password").expect("reopen vault");
        let restored = reopened
            .load_filesystem_store()
            .expect("load filesystem store");
        assert_eq!(restored.files().len(), 1);
        assert_eq!(restored.files()[0].name, "readme.md");
    }

    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn sdk_secret_store_persists_changes() {
    let path = temp_vault_path("secrets_feature");
    let root = path.parent().expect("temp root").to_path_buf();

    let client = VaultClient::new();
    client
        .create_with(&path, b"password", CreateVaultOptions::default())
        .expect("create vault");

    {
        let mut handle = client.open(&path, b"password").expect("open vault");
        let mut store = handle.load_secret_store().expect("load secret store");

        let entry = LoginEntry::new(
            Some("/accounts".to_string()),
            "mail".to_string(),
            "alex".to_string(),
            EncryptedField::new(vec![1, 2, 3, 4]),
            Some("https://mail.example".to_string()),
            None,
            None,
        )
        .expect("build secret");

        store.insert(entry).expect("insert secret");

        let wrote = handle
            .commit_secret_store(&mut store)
            .expect("commit secret store");
        assert!(wrote);
    }

    {
        let mut reopened = client.open(&path, b"password").expect("reopen vault");
        let restored = reopened.load_secret_store().expect("load secret store");
        assert_eq!(restored.list_all().len(), 1);
        assert_eq!(restored.list_all()[0].name, "mail");
    }

    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn sdk_centralized_stores_commit_roundtrip() {
    let path = temp_vault_path("centralized_stores");
    let root = path.parent().expect("temp root").to_path_buf();

    let client = VaultClient::new();
    client
        .create_with(&path, b"password", CreateVaultOptions::default())
        .expect("create vault");

    {
        let mut handle = client.open(&path, b"password").expect("open vault");

        {
            let stores = handle.load_stores().expect("load stores");

            let file = scan_file(Path::new("todo.txt")).expect("scan file");
            stores.filesystem.add_file(file).expect("add file");

            let entry = LoginEntry::new(
                Some("/work".to_string()),
                "tracker".to_string(),
                "alex".to_string(),
                EncryptedField::new(vec![9, 9, 9]),
                None,
                None,
                None,
            )
            .expect("build secret");
            stores.secrets.insert(entry).expect("insert secret");
        }

        let commit = handle.commit_stores().expect("commit stores");
        assert!(commit.filesystem);
        assert!(commit.secrets);
    }

    {
        let mut reopened = client.open(&path, b"password").expect("reopen vault");
        let stores = reopened.load_stores().expect("load stores");
        assert_eq!(stores.filesystem.files().len(), 1);
        assert_eq!(stores.filesystem.files()[0].name, "todo.txt");
        assert_eq!(stores.secrets.list_all().len(), 1);
        assert_eq!(stores.secrets.list_all()[0].name, "tracker");
    }

    fs::remove_dir_all(root).expect("cleanup");
}
