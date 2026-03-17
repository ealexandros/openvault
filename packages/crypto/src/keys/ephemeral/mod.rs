mod private;
mod public;

pub use private::EphemeralPrivateKey;
pub use public::EphemeralPublicKey;

use argon2::password_hash::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::errors::Result;

pub const EPHEMERAL_KEY_SIZE: usize = 32;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct EphemeralKeyPair {
    pub public: EphemeralPublicKey,
    pub private: EphemeralPrivateKey,
}

impl EphemeralKeyPair {
    pub fn new(public: EphemeralPublicKey, private: EphemeralPrivateKey) -> Self {
        Self { public, private }
    }

    pub fn generate() -> Result<Self> {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = X25519PublicKey::from(&secret);

        Ok(Self {
            public: EphemeralPublicKey(public.to_bytes()),
            private: EphemeralPrivateKey::from_bytes(secret.to_bytes())?,
        })
    }
}

impl fmt::Debug for EphemeralKeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EphemeralKeyPair")
            .field("public", &self.public)
            .field("private", &"<redacted>")
            .finish()
    }
}
