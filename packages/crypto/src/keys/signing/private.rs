use std::fmt;

use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::errors::{Error, Result};
use crate::internal::secure_memory::SecureMemory;
use crate::keys::signing::SIGNING_KEY_SIZE;

pub type SigningPrivateKeyType = [u8; SIGNING_KEY_SIZE];

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SigningPrivateKey {
    key: SecureMemory<SIGNING_KEY_SIZE>,
}

impl SigningPrivateKey {
    pub fn from_bytes(bytes: SigningPrivateKeyType) -> Result<Self> {
        let key = SecureMemory::new(bytes).map_err(|_| Error::MemoryLockFailed)?;
        Ok(Self { key })
    }

    pub fn as_bytes(&self) -> &SigningPrivateKeyType {
        self.key.as_ref()
    }
}

impl Clone for SigningPrivateKey {
    fn clone(&self) -> Self {
        Self::from_bytes(*self.as_bytes())
            .expect("SigningPrivateKey clone failed: memory lock exhausted")
    }
}

impl PartialEq for SigningPrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for SigningPrivateKey {}

impl Serialize for SigningPrivateKey {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_bytes().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SigningPrivateKey {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = SigningPrivateKeyType::deserialize(deserializer)?;
        Self::from_bytes(bytes).map_err(DeError::custom)
    }
}

impl fmt::Debug for SigningPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SigningPrivateKey(<redacted>)")
    }
}
