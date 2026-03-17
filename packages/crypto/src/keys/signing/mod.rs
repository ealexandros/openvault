mod private;
mod public;

pub use private::SigningPrivateKey;
pub use public::SigningPublicKey;

use argon2::password_hash::rand_core::OsRng;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::errors::Result;

pub const SIGNING_KEY_SIZE: usize = 32;
pub type SignatureKeyType = [u8; SIGNING_KEY_SIZE];

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct SigningKeyPair {
    pub public: SigningPublicKey,
    pub private: SigningPrivateKey,
}

impl SigningKeyPair {
    pub fn new(public: SigningPublicKey, private: SigningPrivateKey) -> Self {
        Self { public, private }
    }

    pub fn generate() -> Result<Self> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        Ok(Self {
            public: SigningPublicKey(verifying_key.to_bytes()),
            private: SigningPrivateKey::from_bytes(signing_key.to_bytes())?,
        })
    }
}

impl fmt::Debug for SigningKeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigningKeyPair")
            .field("public", &self.public)
            .field("private", &"<redacted>")
            .finish()
    }
}
