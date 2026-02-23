use openvault_crypto::encryption::nonce::Nonce;
use openvault_crypto::encryption::xchacha20::XChaCha20Poly1305Cipher;
use openvault_crypto::encryption::{Cipher, EncryptionAlgorithm};
use std::io::Cursor;
use std::str::FromStr;

const KEY_SIZE: usize = 32;

#[test]
fn test_xchacha20_roundtrip() {
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve().unwrap();
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
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve().unwrap();
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

    let cipher = algo.resolve().unwrap();
    assert!(
        cipher
            .encrypt_prefixed_nonce(&[0u8; 32], b"test", b"")
            .is_ok()
    );
}

#[test]
fn test_xchacha20_streaming() {
    let cipher = XChaCha20Poly1305Cipher;
    let key = [42u8; KEY_SIZE];
    let data = b"Large-ish data to test streaming encryption functionality.".repeat(1000);

    let mut input = Cursor::new(data.clone());
    let mut ciphertext_stream = Cursor::new(Vec::new());

    cipher
        .encrypt_stream(&key, &mut input, &mut ciphertext_stream)
        .unwrap();

    let mut encrypted_input = Cursor::new(ciphertext_stream.into_inner());
    let mut output_stream = Cursor::new(Vec::new());

    cipher
        .decrypt_stream(&key, &mut encrypted_input, &mut output_stream)
        .unwrap();

    let output = output_stream.into_inner();
    assert_eq!(data, output);
}

#[test]
fn test_xchacha20_streaming_bad_aad() {
    let cipher = XChaCha20Poly1305Cipher;
    let key = [42u8; KEY_SIZE];
    let data = b"Large-ish data to test streaming encryption functionality.".repeat(10);

    let mut input = Cursor::new(data);
    let mut ciphertext_stream = Cursor::new(Vec::new());

    cipher
        .encrypt_stream(&key, &mut input, &mut ciphertext_stream)
        .unwrap();

    let ciphertext = ciphertext_stream.into_inner();

    let mut corrupted = Vec::new();
    corrupted.extend_from_slice(&ciphertext[..24]);
    corrupted.push(0u8);
    corrupted.extend_from_slice(&ciphertext[24..]);

    let mut encrypted_input = Cursor::new(corrupted);
    let mut output_stream = Cursor::new(Vec::new());

    let result = cipher.decrypt_stream(&key, &mut encrypted_input, &mut output_stream);

    assert!(result.is_err());
}

#[test]
fn test_xchacha20_incorrect_key() {
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve().unwrap();
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
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve().unwrap();
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
    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.resolve().unwrap();
    let key = [42u8; KEY_SIZE];
    let plaintext = b"";

    let encrypted_blob = cipher.encrypt_prefixed_nonce(&key, plaintext, b"").unwrap();
    let decrypted = cipher
        .decrypt_prefixed_nonce(&key, &encrypted_blob, b"")
        .unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}
