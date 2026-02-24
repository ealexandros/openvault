use std::fs;
use std::path::PathBuf;

use openvault_core::errors::Error;
use openvault_core::operations::config::{CreateConfig, OpenConfig};
use openvault_core::operations::replay::replay_since_checkpoint;
use openvault_core::operations::vault::{create_vault_with, open_vault};
use uuid::Uuid;

fn temp_vault_path(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("openvault_vault_tests_{}", Uuid::new_v4()));
    fs::create_dir_all(&dir).expect("create temp directory");
    dir.join(format!("{name}.ov"))
}

#[test]
fn open_vault_with_wrong_password_returns_unlock_failed() {
    let path = temp_vault_path("unlock_failed");
    let root = path.parent().expect("temp root").to_path_buf();

    create_vault_with(&path, b"correct-password", CreateConfig::default()).expect("create vault");

    let result = open_vault(&path, b"wrong-password", OpenConfig::default());
    assert!(matches!(result, Err(Error::UnlockFailed)));

    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn replay_since_checkpoint_returns_structured_state() {
    let path = temp_vault_path("replay_state");
    let root = path.parent().expect("temp root").to_path_buf();

    create_vault_with(&path, b"password", CreateConfig::default()).expect("create vault");

    let mut session =
        open_vault(&path, b"password", OpenConfig::default()).expect("open vault session");
    let replay = replay_since_checkpoint(&mut session).expect("replay");

    assert!(replay.checkpoint.is_none());
    assert!(replay.records.is_empty());
    assert_eq!(replay.subheader.checkpoint_offset, 0);
    assert_eq!(replay.subheader.tail_record_offset, 0);
    assert_eq!(replay.subheader.last_sequence, 0);

    drop(session);
    fs::remove_dir_all(root).expect("cleanup");
}
