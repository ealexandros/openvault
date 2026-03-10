use serde::{Deserialize, Serialize};

use crate::compression::CompressionAlgorithm;
use crate::encryption::EncryptionAlgorithm;
use crate::errors::{Error, Result};

pub const ENVELOPE_VERSION: u8 = 1;

// @todo-now rename this

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256 = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    Ed25519 = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum KdfAlgorithm {
    HkdfSha256 = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionMode {
    None = 0,
    Zstd = 1,
}

impl CompressionMode {
    pub fn compress(self, data: &[u8]) -> Result<Vec<u8>> {
        match self {
            CompressionMode::None => Ok(data.to_vec()),
            CompressionMode::Zstd => CompressionAlgorithm::Zstd.resolve().compress(data),
        }
    }

    pub fn decompress(self, data: &[u8]) -> Result<Vec<u8>> {
        match self {
            CompressionMode::None => Ok(data.to_vec()),
            CompressionMode::Zstd => CompressionAlgorithm::Zstd.resolve().decompress(data),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeHeader {
    pub version: u8,
    pub hash: HashAlgorithm,
    pub signature: SignatureAlgorithm,
    pub kdf: KdfAlgorithm,
    pub encryption: EncryptionAlgorithm,
    pub compression: CompressionMode,
    pub ephemeral_public_key: [u8; 32],
}

impl EnvelopeHeader {
    pub fn aad_bytes(&self) -> [u8; 38] {
        let mut aad = [0u8; 38];
        aad[0] = self.version;
        aad[1] = self.hash as u8;
        aad[2] = self.signature as u8;
        aad[3] = self.kdf as u8;
        aad[4] = self.encryption as u8;
        aad[5] = self.compression as u8;
        aad[6..].copy_from_slice(&self.ephemeral_public_key);
        aad
    }

    pub fn ensure_supported(&self) -> Result<()> {
        if self.version != ENVELOPE_VERSION {
            return Err(Error::UnsupportedEnvelopeVersion(self.version));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub header: EnvelopeHeader,
    pub ciphertext: Vec<u8>,
}
