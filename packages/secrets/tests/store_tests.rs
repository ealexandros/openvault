use openvault_secrets::domain::records::SecretsChange;
use openvault_secrets::domain::secrets::crypto::EncryptedField;
use openvault_secrets::domain::secrets::login::LoginEntry;
use openvault_secrets::domain::store::SecretStore;

#[test]
fn test_vault_delta_tracking() {
    let mut vault = SecretStore::new();

    let entry = LoginEntry::new(
        None,
        "test".into(),
        "user".into(),
        EncryptedField::new(vec![1, 2, 3]),
        None,
        None,
        None,
    )
    .unwrap();
    vault.insert(entry).unwrap();

    match vault.pending_changes() {
        Some(SecretsChange::Deltas(deltas)) => {
            assert_eq!(deltas.len(), 1);
        }
        _ => panic!("Expected deltas"),
    }

    vault.reset_sync_state();
    assert!(vault.pending_changes().is_none());
}

#[test]
fn test_vault_snapshot_trigger() {
    let mut vault = SecretStore::new();

    for i in 0..30 {
        let entry = LoginEntry::new(
            None,
            format!("test{}", i),
            "user".into(),
            EncryptedField::new(vec![1, 2, 3]),
            None,
            None,
            None,
        )
        .unwrap();
        vault.insert(entry).unwrap();
    }

    match vault.pending_changes() {
        Some(SecretsChange::Snapshot(snapshot)) => {
            assert_eq!(snapshot.entries.len(), 30);
        }
        _ => panic!("Expected snapshot"),
    }
}

#[test]
fn test_vault_restore() {
    let mut vault = SecretStore::new();
    let entry = LoginEntry::new(
        None,
        "test".into(),
        "user".into(),
        EncryptedField::new(vec![1, 2, 3]),
        None,
        None,
        None,
    )
    .unwrap();

    vault.insert(entry.clone()).unwrap();

    let change = vault.create_snapshot();
    let snapshot = match change {
        SecretsChange::Snapshot(s) => s,
        _ => panic!("Not a snapshot"),
    };

    let restored_vault = SecretStore::restore(snapshot, vec![]).unwrap();
    assert_eq!(restored_vault.list_all().len(), 1);
    assert_eq!(
        restored_vault.get_entry(&entry.id).unwrap().username,
        "user"
    );
}

#[test]
fn test_vault_folder_contents() {
    let mut vault = SecretStore::new();

    let root_entry = LoginEntry::new(
        Some("/".into()),
        "root".into(),
        "root_user".into(),
        EncryptedField::new(vec![1]),
        None,
        None,
        None,
    )
    .unwrap();
    vault.insert(root_entry).unwrap();

    let work_entry = LoginEntry::new(
        Some("/work".into()),
        "work_secret".into(),
        "work_user".into(),
        EncryptedField::new(vec![2]),
        None,
        None,
        None,
    )
    .unwrap();
    vault.insert(work_entry).unwrap();

    let work_team_entry = LoginEntry::new(
        Some("/work/team".into()),
        "team_secret".into(),
        "team_user".into(),
        EncryptedField::new(vec![3]),
        None,
        None,
        None,
    )
    .unwrap();
    vault.insert(work_team_entry).unwrap();

    let personal_entry = LoginEntry::new(
        Some("/personal/private".into()),
        "personal_secret".into(),
        "personal_user".into(),
        EncryptedField::new(vec![4]),
        None,
        None,
        None,
    )
    .unwrap();
    vault.insert(personal_entry).unwrap();

    let root_entries = vault.list_by_folder("/");
    let root_subfolders: Vec<_> = vault.list_subfolders("/").into_iter().collect();
    assert_eq!(root_entries.len(), 1);
    assert_eq!(root_entries[0].name, "root");
    assert_eq!(root_subfolders.len(), 2);
    assert_eq!(root_subfolders[0], "/personal");
    assert_eq!(root_subfolders[1], "/work");

    let work_entries = vault.list_by_folder("/work");
    let work_subfolders: Vec<_> = vault.list_subfolders("/work").into_iter().collect();
    assert_eq!(work_entries.len(), 1);
    assert_eq!(work_entries[0].name, "work_secret");
    assert_eq!(work_subfolders.len(), 1);
    assert_eq!(work_subfolders[0], "/work/team");
}

#[test]
fn test_vault_update_rejects_duplicate_name_after_folder_normalization() {
    let mut vault = SecretStore::new();

    let work_entry = LoginEntry::new(
        Some("/work".into()),
        "github".into(),
        "work_user".into(),
        EncryptedField::new(vec![1]),
        None,
        None,
        None,
    )
    .unwrap();
    let work_id = work_entry.id;
    vault.insert(work_entry).unwrap();

    let personal_entry = LoginEntry::new(
        Some("/personal".into()),
        "github".into(),
        "personal_user".into(),
        EncryptedField::new(vec![2]),
        None,
        None,
        None,
    )
    .unwrap();
    let personal_id = personal_entry.id;
    vault.insert(personal_entry).unwrap();

    let err = vault.update(
        personal_id,
        openvault_secrets::LoginEntryPatch {
            folder: Some("work/".into()),
            ..Default::default()
        },
    );

    assert!(err.is_err());
    assert_eq!(vault.get_entry(&work_id).unwrap().folder, "/work");
}
