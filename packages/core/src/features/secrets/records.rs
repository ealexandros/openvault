use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::{LoginEntry, LoginEntryPatch};

pub const SECRETS_WIRE_VERSION_V1: u16 = 1;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SecretSnapshot {
    pub entries: HashMap<Uuid, LoginEntry>,
}

impl SecretSnapshot {
    pub const fn new(entries: HashMap<Uuid, LoginEntry>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecretDelta {
    Added(LoginEntry),
    Updated { id: Uuid, patch: LoginEntryPatch },
    Deleted { id: Uuid },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecretsChange {
    Snapshot(SecretSnapshot),
    Deltas(Vec<SecretDelta>),
}
