use argon2::password_hash::rand_core::OsRng;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub type SignatureKeyType = [u8; 32];

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct SigningPrivateKey(SignatureKeyType);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct SigningPublicKey(SignatureKeyType);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SigningKeyPair {
    pub public: SigningPublicKey,
    pub private: SigningPrivateKey,
}

impl SigningPrivateKey {
    pub fn from_bytes(bytes: SignatureKeyType) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &SignatureKeyType {
        &self.0
    }
}

impl SigningPublicKey {
    pub fn from_bytes(bytes: SignatureKeyType) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &SignatureKeyType {
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

impl fmt::Debug for SigningPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SigningPrivateKey(<redacted>)")
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

impl Zeroize for SigningKeyPair {
    fn zeroize(&mut self) {
        self.public.zeroize();
        self.private.zeroize();
    }
}
