use serde::{Deserialize, Serialize, de::DeserializeOwned};
use zeroize::{Zeroize, ZeroizeOnDrop};

use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::derived_key::DerivedKey;

use super::super::error::{Result, SecretError};

#[derive(Clone, Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, PartialEq, Eq)]
pub struct SealedValue(Vec<u8>);

impl SealedValue {
    pub const fn new(ciphertext: Vec<u8>) -> Self {
        Self(ciphertext)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn seal_bytes(
        plaintext: &[u8],
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        let encrypted = cipher
            .resolve()
            .encrypt_prefixed_nonce(key.as_bytes(), plaintext, b"")
            .map_err(|e| SecretError::CryptoError(e.to_string()))?;

        Ok(Self(encrypted))
    }

    pub fn reveal_bytes(&self, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<Vec<u8>> {
        cipher
            .resolve()
            .decrypt_prefixed_nonce(key.as_bytes(), &self.0, b"")
            .map_err(|e| SecretError::CryptoError(e.to_string()))
    }

    pub fn seal_string(
        plaintext: impl AsRef<str>,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        Self::seal_bytes(plaintext.as_ref().as_bytes(), key, cipher)
    }

    pub fn reveal_string(&self, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<String> {
        let bytes = self.reveal_bytes(key, cipher)?;
        String::from_utf8(bytes).map_err(|e| SecretError::InvalidInput(e.to_string()))
    }

    pub fn seal_value<T: Serialize>(
        value: &T,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        let payload = postcard::to_allocvec(value)
            .map_err(|e| SecretError::SerializationError(e.to_string()))?;
        Self::seal_bytes(&payload, key, cipher)
    }

    pub fn reveal_value<T: DeserializeOwned>(
        &self,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<T> {
        let payload = self.reveal_bytes(key, cipher)?;
        postcard::from_bytes(&payload).map_err(|e| SecretError::DeserializationError(e.to_string()))
    }
}
