use openvault_core::crypto::kdf;

#[test]
fn test_kdf_consistency() {
    let password = b"password123";
    let salt = kdf::generate_default_salt();
    let key1 = kdf::derive_master_key(password, &salt).unwrap();
    let key2 = kdf::derive_master_key(password, &salt).unwrap();

    assert_eq!(key1, key2);
}

#[test]
fn test_kdf_random_salt() {
    let password = b"password123";
    let salt1 = kdf::generate_default_salt();
    let key1 = kdf::derive_master_key(password, &salt1).unwrap();
    let salt2 = kdf::generate_default_salt();
    let key2 = kdf::derive_master_key(password, &salt2).unwrap();

    assert_ne!(key1, key2);
    assert_ne!(salt1, salt2);
}
