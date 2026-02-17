use openvault_crypto::keys::MasterKey;
use openvault_secrets::SecretManager;
use openvault_secrets::manager::params::{AddSecretEntryParams, UpdateSecretEntryParams};
use uuid::Uuid;

#[test]
fn test_secret_manager_basic_ops() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key);

    let params = AddSecretEntryParams {
        name: "test_service".to_string(),
        username: "user1".to_string(),
        password: "pass1".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };

    let entry_id = manager.add(params).unwrap();

    let retrieved = manager.get(&entry_id).unwrap();
    assert_eq!(retrieved.username, "user1");

    let password = manager.reveal_password(&entry_id).unwrap();
    assert_eq!(password, "pass1".to_string());

    let list = manager.list();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "test_service");

    let update = UpdateSecretEntryParams {
        name: None,
        username: Some("user1_updated".to_string()),
        password: Some("pass1_updated".to_string()),
        website: None,
        comments: None,
        totp: None,
    };
    manager.update(&entry_id, update).unwrap();

    let updated = manager.get(&entry_id).unwrap();
    assert_eq!(updated.username, "user1_updated");

    let password = manager.reveal_password(&entry_id).unwrap();
    assert_eq!(password, "pass1_updated");

    manager.delete(&entry_id).unwrap();
    assert!(manager.get(&entry_id).is_none());
    assert_eq!(manager.list().len(), 0);
}

#[test]
fn test_secret_manager_persistence() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key.clone());

    let params = AddSecretEntryParams {
        name: "service".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };

    let entry_id1 = manager.add(params).unwrap();

    let chunk1 = manager.export_changes().unwrap();
    manager.clear_deltas();

    let params2 = AddSecretEntryParams {
        name: "service2".to_string(),
        username: "user2".to_string(),
        password: "pass2".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };
    let entry_id2 = manager.add(params2).unwrap();

    let chunk2 = manager.export_changes().unwrap();

    let chunks = vec![chunk1, chunk2];
    let new_manager = SecretManager::unlock(key, chunks).unwrap();

    assert_eq!(new_manager.list().len(), 2);
    assert_eq!(new_manager.get(&entry_id1).unwrap().username, "user");
    assert_eq!(new_manager.get(&entry_id2).unwrap().username, "user2");
}

#[test]
fn test_secret_manager_snapshot() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key.clone());

    let mut last_entry_id = Uuid::default();

    for i in 0..35 {
        let params = AddSecretEntryParams {
            name: format!("service_{}", i),
            username: format!("user_{}", i),
            password: format!("pass_{}", i),
            website: "".to_string(),
            comments: "".to_string(),
            totp: None,
        };
        last_entry_id = manager.add(params).unwrap();
    }

    let chunk = manager.export_changes().unwrap();

    let new_manager = SecretManager::unlock(key, vec![chunk]).unwrap();

    assert_eq!(new_manager.list().len(), 35);
    assert_eq!(new_manager.get(&last_entry_id).unwrap().username, "user_34");
}

#[test]
fn test_validation_errors() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key);

    let params_err = AddSecretEntryParams {
        name: "".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };
    assert!(manager.add(params_err).is_err());

    let params = AddSecretEntryParams {
        name: "dup".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };
    manager.add(params).unwrap();

    let params_dup = AddSecretEntryParams {
        name: "dup".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };

    let err = manager.add(params_dup);
    assert!(err.is_err());
}
