use argon2::password_hash::rand_core::OsRng;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct EphemeralPrivateKey([u8; 32]);

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct EphemeralPublicKey([u8; 32]);

#[derive(Clone)]
pub struct EphemeralKeyPair {
    pub public: EphemeralPublicKey,
    pub private: EphemeralPrivateKey,
}

impl EphemeralPrivateKey {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn shared_secret(&self, peer: &EphemeralPublicKey) -> [u8; 32] {
        let secret = StaticSecret::from(self.0);
        let peer_public = X25519PublicKey::from(peer.0);
        let shared = secret.diffie_hellman(&peer_public);
        shared.to_bytes()
    }
}

impl EphemeralPublicKey {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_bytes(&self) -> [u8; 32] {
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

impl Zeroize for EphemeralKeyPair {
    fn zeroize(&mut self) {
        self.public.zeroize();
        self.private.zeroize();
    }
}
