use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::encryption::{Cipher, xchacha20};
use crate::errors::{Error, Result};

pub type CipherRef = &'static dyn Cipher;

#[repr(u8)]
#[derive(
    Debug, PartialEq, EnumString, Display, AsRefStr, Copy, Clone, Serialize, Deserialize, Default,
)]
#[strum(serialize_all = "lowercase")]
pub enum EncryptionAlgorithm {
    #[default]
    #[strum(serialize = "xchacha20poly1305", serialize = "xchacha20")]
    XChaCha20Poly1305 = 1,
}

impl EncryptionAlgorithm {
    pub fn resolve(&self) -> Result<CipherRef> {
        match self {
            EncryptionAlgorithm::XChaCha20Poly1305 => Ok(&xchacha20::XChaCha20Poly1305Cipher),
        }
    }
}

impl TryFrom<u8> for EncryptionAlgorithm {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            x if x == EncryptionAlgorithm::XChaCha20Poly1305 as u8 => {
                Ok(EncryptionAlgorithm::XChaCha20Poly1305)
            }
            _ => Err(Error::UnsupportedCipher(value)),
        }
    }
}
