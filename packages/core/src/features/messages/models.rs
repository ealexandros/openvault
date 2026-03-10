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
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct MessageContact {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    pub signing_pub_key: SigningPublicKey,
    pub ephemeral_pub_key: EphemeralPublicKey,
    pub secure: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageCredentialsView {
    pub name: String,
    pub signing_pub_key: SigningPublicKey,
    pub ephemeral_pub_key: EphemeralPublicKey,
    pub secure: bool,
    pub expires_at: Option<DateTime<Utc>>,
}

impl MessageCredentials {
    pub fn new(
        name: String,
        signing_keys: SigningKeyPair,
        ephemeral_keys: EphemeralKeyPair,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            name,
            signing_keys,
            ephemeral_keys,
            expires_at,
            created_at: Utc::now(),
        }
    }

    pub fn to_view(&self) -> MessageCredentialsView {
        MessageCredentialsView {
            name: self.name.clone(),
            signing_pub_key: self.signing_keys.public.clone(),
            ephemeral_pub_key: self.ephemeral_keys.public.clone(),
            secure: true,
            expires_at: self.expires_at,
        }
    }
}

impl MessageContact {
    pub fn new(
        name: String,
        signing_pub_key: SigningPublicKey,
        ephemeral_pub_key: EphemeralPublicKey,
        secure: bool,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            signing_pub_key,
            ephemeral_pub_key,
            secure,
            expires_at,
            created_at: Utc::now(),
        }
    }
}

impl Zeroize for MessageCredentials {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.signing_keys.zeroize();
        self.ephemeral_keys.zeroize();
        self.expires_at = None;
    }
}

impl Zeroize for MessageContact {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.signing_pub_key.zeroize();
        self.ephemeral_pub_key.zeroize();
        self.expires_at = None;
    }
}
