use openvault_crypto::keys::MasterKey;
use openvault_secrets::SecretManager;
use openvault_secrets::manager::params::{AddSecretEntryParams, UpdateSecretEntryParams};
use uuid::Uuid;

#[test]
fn test_secret_manager_basic_ops() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key);

    let params = AddSecretEntryParams {
        folder: "work/team".to_string(),
        name: "test_service".to_string(),
        username: "user1".to_string(),
        password: "pass1".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };

    let entry_id = manager.insert(params).unwrap();

    let retrieved = manager.get_entry(&entry_id).unwrap();
    assert_eq!(retrieved.username, "user1");

    let password = manager.show_password(&entry_id).unwrap();
    assert_eq!(password, "pass1".to_string());

    let list = manager.list_all();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "test_service");
    assert_eq!(list[0].folder, "/work/team");

    let update = UpdateSecretEntryParams {
        folder: Some("personal/private".to_string()),
        name: None,
        username: Some("user1_updated".to_string()),
        password: Some("pass1_updated".to_string()),
        website: None,
        comments: None,
        totp: None,
    };
    manager.update(&entry_id, update).unwrap();

    let updated = manager.get_entry(&entry_id).unwrap();
    assert_eq!(updated.username, "user1_updated");
    assert_eq!(updated.folder, "/personal/private");

    let password = manager.show_password(&entry_id).unwrap();
    assert_eq!(password, "pass1_updated");

    manager.delete(&entry_id).unwrap();
    assert!(manager.get_entry(&entry_id).is_none());
    assert_eq!(manager.list_all().len(), 0);
}

#[test]
fn test_secret_manager_defaults_folder_to_root() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key);

    let params = AddSecretEntryParams {
        folder: "".to_string(),
        name: "root_service".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };

    let entry_id = manager.insert(params).unwrap();
    let entry = manager.get_entry(&entry_id).unwrap();

    assert_eq!(entry.folder, "/");
}

#[test]
fn test_secret_manager_persistence() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key.clone());

    let params = AddSecretEntryParams {
        folder: "".to_string(),
        name: "service".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };

    let entry_id1 = manager.insert(params).unwrap();

    let chunk1 = manager.export_changes().unwrap();
    manager.reset_sync_state();

    let params2 = AddSecretEntryParams {
        folder: "".to_string(),
        name: "service2".to_string(),
        username: "user2".to_string(),
        password: "pass2".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };
    let entry_id2 = manager.insert(params2).unwrap();

    let chunk2 = manager.export_changes().unwrap();

    let chunks = vec![chunk1, chunk2];
    let new_manager = SecretManager::unlock(key, chunks).unwrap();

    assert_eq!(new_manager.list_all().len(), 2);
    assert_eq!(new_manager.get_entry(&entry_id1).unwrap().username, "user");
    assert_eq!(new_manager.get_entry(&entry_id2).unwrap().username, "user2");
}

#[test]
fn test_secret_manager_snapshot() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key.clone());

    let mut last_entry_id = Uuid::default();

    for i in 0..35 {
        let params = AddSecretEntryParams {
            folder: "".to_string(),
            name: format!("service_{}", i),
            username: format!("user_{}", i),
            password: format!("pass_{}", i),
            website: "".to_string(),
            comments: "".to_string(),
            totp: None,
        };
        last_entry_id = manager.insert(params).unwrap();
    }

    let chunk = manager.export_changes().unwrap();

    let new_manager = SecretManager::unlock(key, vec![chunk]).unwrap();

    assert_eq!(new_manager.list_all().len(), 35);
    assert_eq!(
        new_manager.get_entry(&last_entry_id).unwrap().username,
        "user_34"
    );
}

#[test]
fn test_validation_errors() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key);

    let params_err = AddSecretEntryParams {
        folder: "".to_string(),
        name: "".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };
    assert!(manager.insert(params_err).is_err());

    let params = AddSecretEntryParams {
        folder: "".to_string(),
        name: "dup".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };
    manager.insert(params).unwrap();

    let params_dup = AddSecretEntryParams {
        folder: "".to_string(),
        name: "dup".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    };

    let err = manager.insert(params_dup);
    assert!(err.is_err());
}

#[test]
fn test_same_name_allowed_in_different_folders() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key);

    manager
        .insert(AddSecretEntryParams {
            folder: "/work".to_string(),
            name: "github".to_string(),
            username: "user1".to_string(),
            password: "pass1".to_string(),
            website: "".to_string(),
            comments: "".to_string(),
            totp: None,
        })
        .unwrap();

    let second = manager.insert(AddSecretEntryParams {
        folder: "/personal".to_string(),
        name: "github".to_string(),
        username: "user2".to_string(),
        password: "pass2".to_string(),
        website: "".to_string(),
        comments: "".to_string(),
        totp: None,
    });

    assert!(second.is_ok());
}

#[test]
fn test_manager_folder_contents_api() {
    let key = MasterKey::new([0u8; 32]);
    let mut manager = SecretManager::create(key);

    manager
        .insert(AddSecretEntryParams {
            folder: "/work".to_string(),
            name: "secret_a".to_string(),
            username: "user_a".to_string(),
            password: "pass_a".to_string(),
            website: "".to_string(),
            comments: "".to_string(),
            totp: None,
        })
        .unwrap();

    manager
        .insert(AddSecretEntryParams {
            folder: "/work/team".to_string(),
            name: "secret_b".to_string(),
            username: "user_b".to_string(),
            password: "pass_b".to_string(),
            website: "".to_string(),
            comments: "".to_string(),
            totp: None,
        })
        .unwrap();

    let listing = manager.browse_folder("/work");

    assert_eq!(listing.current_folder, "/work");
    assert_eq!(listing.subfolders.len(), 1);
    assert_eq!(listing.subfolders[0].path, "/work/team");
    assert_eq!(listing.entries.len(), 1);
    assert_eq!(listing.entries[0].name, "secret_a");
}
