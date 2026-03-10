use serde::{Deserialize, Serialize};

use crate::compression::CompressionAlgorithm;
use crate::encryption::EncryptionAlgorithm;
use crate::errors::{Error, Result};
use crate::signature::SignatureAlgorithm;

pub const ENVELOPE_VERSION: u8 = 1;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HashAlgorithm {
    #[default]
    Sha256 = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KdfAlgorithm {
    #[default]
    HkdfSha256 = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageHeader {
    pub version: u8,
    pub hash: HashAlgorithm,
    pub kdf: KdfAlgorithm,
    pub signature: SignatureAlgorithm,
    pub encryption: EncryptionAlgorithm,
    pub compression: CompressionAlgorithm,
    pub ephemeral_public_key: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub header: MessageHeader,
    pub ciphertext: Vec<u8>,
}

#[derive(Default)]
pub struct MessageConfig {
    pub hash: HashAlgorithm,
    pub kdf: KdfAlgorithm,
    pub signature: SignatureAlgorithm,
    pub compression: CompressionAlgorithm,
    pub encryption: EncryptionAlgorithm,
}

impl MessageHeader {
    pub fn aad_bytes(&self) -> Vec<u8> {
        let mut aad = vec![
            self.version,
            self.hash as u8,
            self.kdf as u8,
            self.signature as u8,
            self.encryption as u8,
            self.compression as u8,
        ];
        aad.extend_from_slice(&self.ephemeral_public_key);
        aad
    }

    pub fn ensure_supported(&self) -> Result<()> {
        if self.version != ENVELOPE_VERSION {
            return Err(Error::UnsupportedEnvelopeVersion(self.version));
        }

        Ok(())
    }
}
