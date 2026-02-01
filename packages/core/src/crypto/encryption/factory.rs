use super::{Cipher, xchacha20};
use crate::crypto::Result;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum EncryptionAlgorithm {
    #[strum(serialize = "xchacha20poly1305", serialize = "xchacha20")]
    XChaCha20Poly1305,
}

impl EncryptionAlgorithm {
    pub fn get(&self) -> Result<Box<dyn Cipher>> {
        match self {
            EncryptionAlgorithm::XChaCha20Poly1305 => {
                Ok(Box::new(xchacha20::XChaCha20Poly1305Cipher))
            }
        }
    }
}
