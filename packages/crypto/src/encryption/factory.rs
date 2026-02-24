use strum_macros::{AsRefStr, Display, EnumString};

use crate::encryption::{Cipher, xchacha20};
use crate::errors::Result;

pub type CipherRef = &'static dyn Cipher;

#[repr(u8)]
#[derive(Debug, PartialEq, EnumString, Display, AsRefStr, Copy, Clone, Default)]
#[strum(serialize_all = "lowercase")]
pub enum EncryptionAlgorithm {
    #[default]
    #[strum(serialize = "xchacha20poly1305", serialize = "xchacha20")]
    XChaCha20Poly1305,
}

impl EncryptionAlgorithm {
    pub fn resolve(&self) -> Result<CipherRef> {
        match self {
            EncryptionAlgorithm::XChaCha20Poly1305 => Ok(&xchacha20::XChaCha20Poly1305Cipher),
        }
    }
}
