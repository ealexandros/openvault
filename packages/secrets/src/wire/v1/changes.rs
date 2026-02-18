use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::{NonZeroU8, NonZeroU64};
use uuid::Uuid;

pub const WIRE_VERSION_V1: u16 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TotpV1 {
    pub secret: String,
    pub period: NonZeroU64,
    pub digits: NonZeroU8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginEntryV1 {
    pub id: Uuid,
    pub folder: String,
    pub name: String,
    pub username: String,
    pub password: Vec<u8>,
    pub website: String,
    pub comments: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub totp: Option<TotpV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginEntryPatchV1 {
    pub folder: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<Vec<u8>>,
    pub website: Option<String>,
    pub comments: Option<String>,
    pub totp: Option<Option<TotpV1>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SnapshotV1 {
    pub entries: HashMap<Uuid, LoginEntryV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecretDeltaV1 {
    Added(LoginEntryV1),
    Updated { id: Uuid, patch: LoginEntryPatchV1 },
    Deleted { id: Uuid },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecretsChangeV1 {
    Snapshot(SnapshotV1),
    Deltas(Vec<SecretDeltaV1>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretsChunkV1 {
    pub version: u16,
    pub changes: Vec<SecretsChangeV1>,
}

impl SecretsChunkV1 {
    pub fn new(changes: Vec<SecretsChangeV1>) -> Self {
        Self {
            version: WIRE_VERSION_V1,
            changes,
        }
    }
}
