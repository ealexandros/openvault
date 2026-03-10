use argon2::password_hash::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub type EphemeralKeyType = [u8; 32];

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct EphemeralPrivateKey(EphemeralKeyType);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct EphemeralPublicKey(EphemeralKeyType);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EphemeralKeyPair {
    pub public: EphemeralPublicKey,
    pub private: EphemeralPrivateKey,
}

impl EphemeralPrivateKey {
    pub fn from_bytes(bytes: EphemeralKeyType) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &EphemeralKeyType {
        &self.0
    }

    pub fn to_bytes(&self) -> EphemeralKeyType {
        self.0
    }

    pub fn shared_secret(&self, peer: &EphemeralPublicKey) -> EphemeralKeyType {
        let secret = StaticSecret::from(self.0);
        let peer_public = X25519PublicKey::from(peer.0);
        let shared = secret.diffie_hellman(&peer_public);
        shared.to_bytes()
    }
}

impl EphemeralPublicKey {
    pub fn from_bytes(bytes: EphemeralKeyType) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &EphemeralKeyType {
        &self.0
    }

    pub fn to_bytes(&self) -> EphemeralKeyType {
        self.0
    }
}

impl EphemeralKeyPair {
    pub fn new(public: EphemeralPublicKey, private: EphemeralPrivateKey) -> Self {
        Self { public, private }
    }

    pub fn generate() -> Self {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = X25519PublicKey::from(&secret);

        Self {
            public: EphemeralPublicKey(public.to_bytes()),
            private: EphemeralPrivateKey(secret.to_bytes()),
        }
    }
}

impl fmt::Debug for EphemeralPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EphemeralPrivateKey(<redacted>)")
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

impl Zeroize for EphemeralKeyPair {
    fn zeroize(&mut self) {
        self.public.zeroize();
        self.private.zeroize();
    }
}
