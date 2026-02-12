use openvault_crypto::compression::CompressionAlgorithm;
use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::MasterKey;

use crate::errors::{Result, SecretError};

pub fn encrypt(plaintext: &[u8], key: &MasterKey) -> Result<Vec<u8>> {
    let compressor = CompressionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let compressed = compressor
        .compress(plaintext)
        .map_err(|e| SecretError::SerializationError(format!("Compression failed: {}", e)))?;

    let cipher = EncryptionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let encrypted = cipher
        .encrypt_prefixed_nonce(key.as_bytes(), &compressed, b"")
        .map_err(|e| SecretError::SerializationError(format!("Encryption failed: {}", e)))?;

    Ok(encrypted)
}

pub fn decrypt(ciphertext: &[u8], key: &MasterKey) -> Result<Vec<u8>> {
    let cipher = EncryptionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let compressed = cipher
        .decrypt_prefixed_nonce(key.as_bytes(), ciphertext, b"")
        .map_err(|e| SecretError::SerializationError(format!("Decryption failed: {}", e)))?;

    let compressor = CompressionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let plaintext = compressor
        .decompress(&compressed)
        .map_err(|e| SecretError::SerializationError(format!("Decompression failed: {}", e)))?;

    Ok(plaintext)
}

pub fn encrypt_password(data: &[u8], key: &MasterKey) -> Result<Vec<u8>> {
    let cipher = EncryptionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    cipher
        .encrypt_prefixed_nonce(key.as_bytes(), data, b"")
        .map_err(|e| SecretError::SerializationError(format!("Encryption failed: {}", e)))
}

pub fn decrypt_password(data: &[u8], key: &MasterKey) -> Result<String> {
    let cipher = EncryptionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let decrypted = cipher
        .decrypt_prefixed_nonce(key.as_bytes(), data, b"")
        .map_err(|e| SecretError::SerializationError(format!("Decryption failed: {}", e)))?;

    String::from_utf8(decrypted).map_err(|e| SecretError::SerializationError(e.to_string()))
}

pub fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>> {
    postcard::to_allocvec(value).map_err(|e| SecretError::SerializationError(e.to_string()))
}

pub fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    postcard::from_bytes(bytes).map_err(|e| SecretError::DeserializationError(e.to_string()))
}
