use openvault_crypto::compression::factory::CompressionAlgorithm;
use openvault_crypto::encryption::Nonce;
use openvault_crypto::encryption::factory::EncryptionAlgorithm;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::errors::{Error, Result};

#[derive(Debug, Default)]
pub struct Envelope {
    compression: CompressionAlgorithm,
    encryption: EncryptionAlgorithm,
}

impl Envelope {
    pub fn new(compression: CompressionAlgorithm, encryption: EncryptionAlgorithm) -> Self {
        Self {
            compression,
            encryption,
        }
    }

    pub fn seal_bytes(
        &self,
        plaintext: &[u8],
        key: &[u8],
        nonce: &Nonce,
        aad: &[u8],
    ) -> Result<Vec<u8>> {
        let compressor = self.compression.resolve()?;
        let compressed = compressor.compress(plaintext)?;

        let cipher = self.encryption.resolve()?;
        let encrypted = cipher.encrypt(key, nonce, &compressed, aad)?;

        Ok(encrypted)
    }

    pub fn open_bytes(
        &self,
        ciphertext: &[u8],
        key: &[u8],
        nonce: &Nonce,
        aad: &[u8],
    ) -> Result<Vec<u8>> {
        let cipher = self.encryption.resolve()?;
        let compressed = cipher.decrypt(key, nonce, ciphertext, aad)?;

        let compressor = self.compression.resolve()?;
        let plaintext = compressor.decompress(&compressed)?;

        Ok(plaintext)
    }

    pub fn seal<T: Serialize>(
        &self,
        value: &T,
        key: &[u8],
        nonce: &Nonce,
        aad: &[u8],
    ) -> Result<Vec<u8>> {
        let payload = postcard::to_allocvec(value).map_err(|_| Error::InvalidVaultFormat)?;
        self.seal_bytes(&payload, key, nonce, aad)
    }

    pub fn open<T: DeserializeOwned>(
        &self,
        ciphertext: &[u8],
        key: &[u8],
        nonce: &Nonce,
        aad: &[u8],
    ) -> Result<T> {
        let payload = self.open_bytes(ciphertext, key, nonce, aad)?;
        postcard::from_bytes(&payload).map_err(|_| Error::InvalidVaultFormat)
    }
}

pub fn seal<T: Serialize>(value: &T, key: &[u8], nonce: &Nonce, aad: &[u8]) -> Result<Vec<u8>> {
    Envelope::default().seal(value, key, nonce, aad)
}

pub fn open<T: DeserializeOwned>(
    ciphertext: &[u8],
    key: &[u8],
    nonce: &Nonce,
    aad: &[u8],
) -> Result<T> {
    Envelope::default().open(ciphertext, key, nonce, aad)
}
