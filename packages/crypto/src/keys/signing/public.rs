use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::keys::signing::{SIGNING_KEY_SIZE, SignatureKeyType};

pub type SigningPublicKeyType = [u8; SIGNING_KEY_SIZE];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct SigningPublicKey(pub SignatureKeyType);

impl SigningPublicKey {
    pub fn from_bytes(bytes: SigningPublicKeyType) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &SigningPublicKeyType {
        &self.0
    }
}

impl From<SigningPublicKeyType> for SigningPublicKey {
    fn from(bytes: SigningPublicKeyType) -> Self {
        Self(bytes)
    }
}
