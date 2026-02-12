use strum_macros::{AsRefStr, Display, EnumString};

use crate::encryption::{Cipher, xchacha20};
use crate::errors::Result;

#[derive(Debug, PartialEq, EnumString, Display, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum EncryptionAlgorithm {
    #[default]
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
