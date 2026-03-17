use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::keys::ephemeral::EPHEMERAL_KEY_SIZE;

pub type EphemeralPublicKeyType = [u8; EPHEMERAL_KEY_SIZE];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct EphemeralPublicKey(pub EphemeralPublicKeyType);

impl EphemeralPublicKey {
    pub fn from_bytes(bytes: EphemeralPublicKeyType) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &EphemeralPublicKeyType {
        &self.0
    }

    pub fn to_bytes(&self) -> EphemeralPublicKeyType {
        self.0
    }
}

impl From<EphemeralPublicKeyType> for EphemeralPublicKey {
    fn from(bytes: EphemeralPublicKeyType) -> Self {
        Self(bytes)
    }
}
