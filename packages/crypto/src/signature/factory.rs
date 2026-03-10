use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::errors::{Error, Result};
use crate::signature::{Signer, ed25519};

pub type SignerRef = &'static dyn Signer;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, EnumString, Copy, Clone, Serialize, Deserialize, Default)]
#[strum(serialize_all = "lowercase")]
pub enum SignatureAlgorithm {
    #[default]
    Ed25519 = 1,
}

impl SignatureAlgorithm {
    pub fn resolve(self) -> SignerRef {
        match self {
            Self::Ed25519 => &ed25519::Ed25519Signer,
        }
    }
}

impl TryFrom<u8> for SignatureAlgorithm {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Self::Ed25519),
            _ => Err(Error::UnsupportedCompressor(value)),
        }
    }
}
