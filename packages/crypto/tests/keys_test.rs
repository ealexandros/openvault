use openvault_crypto::keys::{DerivedKey, MasterKey, random_salt};

#[test]
fn test_keys_consistency() {
    let password = b"password123";
    let salt = random_salt();
    let key1 = MasterKey::derive(password, &salt).unwrap();
    let key2 = MasterKey::derive(password, &salt).unwrap();

    assert_eq!(key1, key2);
}

#[test]
fn test_keys_random_salt() {
    let password = b"password123";

    let salt1 = random_salt();
    let salt2 = random_salt();
    let key1 = MasterKey::derive(password, &salt1).unwrap();
    let key2 = MasterKey::derive(password, &salt2).unwrap();

    assert_ne!(key1, key2);
    assert_ne!(salt1, salt2);
}

#[test]
fn test_subkey_expansion() {
    let password = b"password123";
    let salt = random_salt();
    let master_key = MasterKey::derive(password, &salt).unwrap();

    let info1 = b"encryption";
    let info2 = b"authentication";

    let subkey1: DerivedKey = master_key.expand(info1).unwrap();
    let subkey2: DerivedKey = master_key.expand(info1).unwrap();
    let subkey3: DerivedKey = master_key.expand(info2).unwrap();

    assert_eq!(subkey1, subkey2);
    assert_ne!(subkey1, subkey3);
}

#[test]
fn test_master_key_as_bytes() {
    let password = b"password123";
    let salt = [0u8; 16];
    let key = MasterKey::derive(password, &salt).unwrap();

    assert_eq!(key.as_bytes().len(), 32);
}
