use openvault_core::crypto::encryption::xchacha20::XChaCha20Poly1305Cipher;
use openvault_core::crypto::encryption::{Cipher, factory::EncryptionAlgorithm};
use std::io::Cursor;
use std::str::FromStr;

const KEY_SIZE: usize = 32;

#[test]
fn test_xchacha20_roundtrip() {
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.get().unwrap();
    let key = [42u8; KEY_SIZE];
    let plaintext = b"Secret message for XChaCha20";

    let encrypted_blob = cipher.encrypt(&key, plaintext).unwrap();
    let decrypted = cipher.decrypt(&key, &encrypted_blob).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}

#[test]
fn test_encryption_factory_from_str() {
    let algo = EncryptionAlgorithm::from_str("xchacha20").unwrap();
    assert_eq!(algo, EncryptionAlgorithm::XChaCha20Poly1305);

    let algo2 = EncryptionAlgorithm::from_str("xchacha20poly1305").unwrap();
    assert_eq!(algo2, EncryptionAlgorithm::XChaCha20Poly1305);

    let cipher = algo.get().unwrap();
    assert!(cipher.encrypt(&[0u8; 32], b"test").is_ok());
}

#[test]
fn test_xchacha20_streaming() {
    let cipher = XChaCha20Poly1305Cipher;
    let key = [42u8; KEY_SIZE];
    let data = b"Large-ish data to test streaming encryption functionality.".repeat(1000);

    let mut input = Cursor::new(&data);
    let mut ciphertext_stream = Vec::new();
    cipher
        .encrypt_stream(&key, &mut input, &mut ciphertext_stream)
        .unwrap();

    let mut output = Vec::new();
    let mut encrypted_input = Cursor::new(&ciphertext_stream);
    cipher
        .decrypt_stream(&key, &mut encrypted_input, &mut output)
        .unwrap();

    assert_eq!(data, output);
}
