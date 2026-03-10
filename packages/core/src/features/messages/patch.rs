use chrono::{DateTime, Utc};
use openvault_crypto::keys::{EphemeralPublicKey, SigningPublicKey};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct MessageContactPatch {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: Option<String>,
    pub signing_pub_key: Option<SigningPublicKey>,
    pub ephemeral_pub_key: Option<EphemeralPublicKey>,
    pub secure: Option<bool>,
    pub expires_at: Option<Option<DateTime<Utc>>>,
}
