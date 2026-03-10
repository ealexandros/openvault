use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

use super::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct PrivateKey(Vec<u8>);

// @todo-now validation, zeroization

impl PrivateKey {
    pub fn new(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

// @todo-now validation, zeroization

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageCredentials {
    pub name: String,
    pub public_key: Vec<u8>,
    pub private_key: PrivateKey,
    pub expiration_at: Option<DateTime<Utc>>,
}

impl MessageCredentials {
    pub fn new(name: String, private_key: Vec<u8>, public_key: Vec<u8>) -> Result<Self> {
        Ok(Self {
            name,
            private_key: PrivateKey::new(private_key)?,
            public_key,
            // @todo-now expiration_at
            expiration_at: None,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageContact {
    pub id: Uuid,
    pub name: String,
    pub expiration_at: Option<DateTime<Utc>>,
    pub secure: bool,
    pub public_key: Vec<u8>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageContactPatch {
    pub name: Option<String>,
    pub expiration_at: Option<Option<DateTime<Utc>>>,
    pub secure: Option<bool>,
    pub public_key: Option<Vec<u8>>,
}

impl MessageContact {
    pub fn new(
        name: String,
        public_key: Vec<u8>,
        secure: bool,
        expiration_at: Option<DateTime<Utc>>,
    ) -> Result<Self> {
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            public_key,
            secure,
            expiration_at,
        })
    }
}

impl Zeroize for MessageCredentials {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.public_key.zeroize();
        self.private_key.zeroize();
        self.expiration_at = None;
    }
}

impl Zeroize for MessageContact {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.public_key.zeroize();
        self.expiration_at = None;
    }
}
