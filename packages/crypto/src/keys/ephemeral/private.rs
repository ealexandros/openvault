use serde::{Deserialize, Deserializer, Serialize, Serializer};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::errors::{Error, Result};
use crate::internal::secure_memory::SecureMemory;
use crate::keys::EphemeralPublicKey;
use crate::keys::ephemeral::EPHEMERAL_KEY_SIZE;

pub type EphemeralPrivateKeyType = [u8; EPHEMERAL_KEY_SIZE];

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct EphemeralPrivateKey {
    key: SecureMemory<EPHEMERAL_KEY_SIZE>,
}

impl EphemeralPrivateKey {
    pub fn from_bytes(bytes: EphemeralPrivateKeyType) -> Result<Self> {
        let key = SecureMemory::new(bytes).map_err(|_| Error::MemoryLockFailed)?;
        Ok(Self { key })
    }

    pub fn as_bytes(&self) -> &EphemeralPrivateKeyType {
        self.key.as_ref()
    }

    pub fn shared_secret(&self, peer: &EphemeralPublicKey) -> EphemeralPrivateKeyType {
        let secret = StaticSecret::from(*self.as_bytes());
        let peer_public = X25519PublicKey::from(peer.0);
        secret.diffie_hellman(&peer_public).to_bytes()
    }
}

impl Clone for EphemeralPrivateKey {
    fn clone(&self) -> Self {
        Self::from_bytes(*self.as_bytes())
            .expect("EphemeralPrivateKey clone failed: memory lock exhausted")
    }
}

impl PartialEq for EphemeralPrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for EphemeralPrivateKey {}

impl Serialize for EphemeralPrivateKey {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_bytes().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EphemeralPrivateKey {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = EphemeralPrivateKeyType::deserialize(deserializer)?;
        Self::from_bytes(bytes).map_err(serde::de::Error::custom)
    }
}
