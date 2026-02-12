use openvault_crypto::compression::factory::CompressionAlgorithm;
use openvault_crypto::encryption::factory::EncryptionAlgorithm;
use openvault_crypto::keys::MasterKey;

use crate::errors::{Result, SecretError};

// @todo-soon codec should be part of the sdk package and based on the version of the vault

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
        .encrypt(key.as_bytes(), &compressed)
        .map_err(|e| SecretError::SerializationError(format!("Encryption failed: {}", e)))?;

    Ok(encrypted)
}

pub fn decrypt(ciphertext: &[u8], key: &MasterKey) -> Result<Vec<u8>> {
    let cipher = EncryptionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let compressed = cipher
        .decrypt(key.as_bytes(), ciphertext)
        .map_err(|e| SecretError::SerializationError(format!("Decryption failed: {}", e)))?;

    let compressor = CompressionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let plaintext = compressor
        .decompress(&compressed)
        .map_err(|e| SecretError::SerializationError(format!("Decompression failed: {}", e)))?;

    Ok(plaintext)
}

pub fn encrypt_password(data: &[u8], key: &MasterKey) -> Result<String> {
    let cipher = EncryptionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let encrypted = cipher
        .encrypt(key.as_bytes(), data)
        .map_err(|e| SecretError::SerializationError(format!("Encryption failed: {}", e)))?;

    String::from_utf8(encrypted).map_err(|e| SecretError::SerializationError(e.to_string()))
}

pub fn decrypt_password(data: &[u8], key: &MasterKey) -> Result<String> {
    let cipher = EncryptionAlgorithm::default()
        .get()
        .map_err(|e| SecretError::SerializationError(e.to_string()))?;
    let decrypted = cipher
        .decrypt(key.as_bytes(), data)
        .map_err(|e| SecretError::SerializationError(format!("Decryption failed: {}", e)))?;

    String::from_utf8(decrypted).map_err(|e| SecretError::SerializationError(e.to_string()))
}

pub fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>> {
    postcard::to_allocvec(value).map_err(|e| SecretError::SerializationError(e.to_string()))
}

pub fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    postcard::from_bytes(bytes).map_err(|e| SecretError::DeserializationError(e.to_string()))
}
