use std::io::Cursor;
use vault_core::crypto::encryption::Cipher;
use vault_core::crypto::encryption::xchacha20::XChaCha20Poly1305Cipher;

const KEY_SIZE: usize = 32;

#[test]
fn test_xchacha20_roundtrip() {
    let cipher = XChaCha20Poly1305Cipher;
    let key = [42u8; KEY_SIZE];
    let plaintext = b"Secret message for XChaCha20";

    let (ciphertext, nonce) = cipher.encrypt(&key, plaintext).unwrap();
    let decrypted = cipher.decrypt(&key, &nonce, &ciphertext).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
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
