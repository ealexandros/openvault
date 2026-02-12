use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::secrets::login::{LoginEntry, LoginEntryPatch};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Snapshot {
    pub entries: HashMap<Uuid, LoginEntry>,
}

impl Snapshot {
    pub fn new(entries: HashMap<Uuid, LoginEntry>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecretDelta {
    Added(LoginEntry),
    Updated { id: Uuid, patch: LoginEntryPatch },
    Deleted { id: Uuid },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecretsChange {
    Snapshot(Snapshot),
    Deltas(Vec<SecretDelta>),
}
