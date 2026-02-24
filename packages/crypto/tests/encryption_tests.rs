use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::encryption::nonce::Nonce;
use std::str::FromStr;

const KEY_SIZE: usize = 32;

#[test]
fn test_xchacha20_roundtrip() {
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve();
    let key = [42u8; KEY_SIZE];
    let plaintext = b"Secret message for XChaCha20";

    let encrypted_blob = cipher.encrypt_prefixed_nonce(&key, plaintext, b"").unwrap();
    let decrypted = cipher
        .decrypt_prefixed_nonce(&key, &encrypted_blob, b"")
        .unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}

#[test]
fn test_xchacha20_explicit_nonce_aad() {
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve();
    let key = [42u8; KEY_SIZE];
    let nonce = Nonce::random();
    let plaintext = b"Secret message";
    let aad = b"context";

    let ciphertext = cipher.encrypt(&key, &nonce, plaintext, aad).unwrap();
    let decrypted = cipher.decrypt(&key, &nonce, &ciphertext, aad).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}

#[test]
fn test_encryption_factory_from_str() {
    let algo = EncryptionAlgorithm::from_str("xchacha20").unwrap();
    assert_eq!(algo, EncryptionAlgorithm::XChaCha20Poly1305);

    let algo2 = EncryptionAlgorithm::from_str("xchacha20poly1305").unwrap();
    assert_eq!(algo2, EncryptionAlgorithm::XChaCha20Poly1305);

    let cipher = algo.resolve();
    assert!(
        cipher
            .encrypt_prefixed_nonce(&[0u8; 32], b"test", b"")
            .is_ok()
    );
}

#[test]
fn test_xchacha20_incorrect_key() {
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve();
    let key1 = [42u8; KEY_SIZE];
    let key2 = [43u8; KEY_SIZE];
    let plaintext = b"Secret message";

    let encrypted_blob = cipher
        .encrypt_prefixed_nonce(&key1, plaintext, b"")
        .unwrap();
    let result = cipher.decrypt_prefixed_nonce(&key2, &encrypted_blob, b"");

    assert!(result.is_err());
}

#[test]
fn test_xchacha20_corrupted_ciphertext() {
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve();
    let key = [42u8; KEY_SIZE];
    let plaintext = b"Secret message";

    let mut encrypted_blob = cipher.encrypt_prefixed_nonce(&key, plaintext, b"").unwrap();

    if let Some(byte) = encrypted_blob.last_mut() {
        *byte ^= 0xFF;
    }

    let result = cipher.decrypt_prefixed_nonce(&key, &encrypted_blob, b"");
    assert!(result.is_err());
}

#[test]
fn test_xchacha20_empty_plaintext() {
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve();
    let key = [42u8; KEY_SIZE];
    let plaintext = b"";

    let encrypted_blob = cipher.encrypt_prefixed_nonce(&key, plaintext, b"").unwrap();
    let decrypted = cipher
        .decrypt_prefixed_nonce(&key, &encrypted_blob, b"")
        .unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}
