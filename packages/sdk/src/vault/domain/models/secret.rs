use chrono::{DateTime, Utc};
use uuid::Uuid;

use zeroize::ZeroizeOnDrop;

#[derive(Debug, Clone, PartialEq, Eq, ZeroizeOnDrop)]
pub struct SecretValue(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Secret {
    pub id: Uuid,
    pub key: String,
    pub value: SecretValue,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
