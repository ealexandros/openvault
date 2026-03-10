use argon2::password_hash::rand_core::OsRng;
use ed25519_dalek::{SigningKey, VerifyingKey};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SigningPrivateKey([u8; 32]);

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SigningPublicKey([u8; 32]);

#[derive(Clone)]
pub struct SigningKeyPair {
    pub public: SigningPublicKey,
    pub private: SigningPrivateKey,
}

impl SigningPrivateKey {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl SigningPublicKey {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl SigningKeyPair {
    pub fn new(public: SigningPublicKey, private: SigningPrivateKey) -> Self {
        Self { public, private }
    }

    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        Self {
            public: SigningPublicKey(verifying_key.to_bytes()),
            private: SigningPrivateKey(signing_key.to_bytes()),
        }
    }
}

impl Zeroize for SigningKeyPair {
    fn zeroize(&mut self) {
        self.public.zeroize();
        self.private.zeroize();
    }
}
