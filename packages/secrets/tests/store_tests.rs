use openvault_secrets::domain::entry::{EncryptedField, SecretEntry};
use openvault_secrets::domain::store::{SecretsChange, SecretsStore};

#[test]
fn test_store_delta_tracking() {
    let mut store = SecretsStore::new();

    let entry = SecretEntry::new(
        "test".into(),
        "user".into(),
        EncryptedField::new(vec![1, 2, 3]),
        None,
        None,
        None,
    )
    .unwrap();
    store.add(entry).unwrap();

    match store.pending_change() {
        Some(SecretsChange::Deltas(deltas)) => {
            assert_eq!(deltas.len(), 1);
        }
        _ => panic!("Expected deltas"),
    }

    store.clear_deltas();
    assert!(store.pending_change().is_none());
}

#[test]
fn test_store_snapshot_trigger() {
    let mut store = SecretsStore::new();

    for i in 0..30 {
        let entry = SecretEntry::new(
            format!("test{}", i),
            "user".into(),
            EncryptedField::new(vec![1, 2, 3]),
            None,
            None,
            None,
        )
        .unwrap();
        store.add(entry).unwrap();
    }

    match store.pending_change() {
        Some(SecretsChange::Snapshot(snapshot)) => {
            assert_eq!(snapshot.entries.len(), 30);
        }
        _ => panic!("Expected snapshot"),
    }
}

#[test]
fn test_store_restore() {
    let mut store = SecretsStore::new();
    let entry = SecretEntry::new(
        "test".into(),
        "user".into(),
        EncryptedField::new(vec![1, 2, 3]),
        None,
        None,
        None,
    )
    .unwrap();

    store.add(entry.clone()).unwrap();

    let change = store.snapshot();
    let snapshot = match change {
        SecretsChange::Snapshot(s) => s,
        _ => panic!("Not a snapshot"),
    };

    let restored_store = SecretsStore::restore(snapshot, vec![]).unwrap();
    assert_eq!(restored_store.list().len(), 1);
    assert_eq!(restored_store.get(&entry.id).unwrap().username, "user");
}
