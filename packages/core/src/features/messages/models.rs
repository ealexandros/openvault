use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use zeroize::Zeroize;

use openvault_crypto::keys::{
    EphemeralKeyPair, EphemeralPublicKey, SigningKeyPair, SigningPublicKey,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct MessageCredentials {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    pub signing_keys: SigningKeyPair,
    pub ephemeral_keys: EphemeralKeyPair,
    pub expiration_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct MessageContact {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    pub signing_key: SigningPublicKey,
    pub ephemeral_key: EphemeralPublicKey,
    pub secure: bool,
    pub expiration_at: Option<DateTime<Utc>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct MessageContactPatch {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: Option<String>,
    pub signing_key: Option<SigningPublicKey>,
    pub ephemeral_key: Option<EphemeralPublicKey>,
    pub secure: Option<bool>,
    pub expiration_at: Option<Option<DateTime<Utc>>>,
}

impl Zeroize for MessageCredentials {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.signing_keys.zeroize();
        self.ephemeral_keys.zeroize();
        self.expiration_at = None;
    }
}

impl Zeroize for MessageContact {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.signing_key.zeroize();
        self.ephemeral_key.zeroize();
        self.expiration_at = None;
    }
}
