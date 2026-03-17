use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

use super::error::SecretError;
use super::models::{LoginEntry, SecretFolder};
use super::patch::{LoginEntryPatch, SecretFolderPatch};

pub const SECRETS_WIRE_VERSION: u16 = 2;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SecretSnapshot {
    pub folders: HashMap<Uuid, SecretFolder>,
    pub entries: HashMap<Uuid, LoginEntry>,
}

impl SecretSnapshot {
    pub fn new(folders: HashMap<Uuid, SecretFolder>, entries: HashMap<Uuid, LoginEntry>) -> Self {
        Self { folders, entries }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecretDelta {
    FolderAdded(SecretFolder),
    FolderDeleted(Uuid),
    FolderUpdated { id: Uuid, patch: SecretFolderPatch },
    EntryAdded(LoginEntry),
    EntryUpdated { id: Uuid, patch: LoginEntryPatch },
    EntryDeleted(Uuid),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecretsChange {
    Snapshot(SecretSnapshot),
    Deltas(Vec<SecretDelta>),
}

impl From<SecretSnapshot> for SecretsChange {
    fn from(value: SecretSnapshot) -> Self {
        Self::Snapshot(value)
    }
}

impl TryFrom<SecretsChange> for SecretSnapshot {
    type Error = SecretError;

    fn try_from(value: SecretsChange) -> Result<Self, Self::Error> {
        match value {
            SecretsChange::Snapshot(snapshot) => Ok(snapshot),
            SecretsChange::Deltas(_) => Err(SecretError::InvalidSnapshot),
        }
    }
}

impl Zeroize for SecretDelta {
    fn zeroize(&mut self) {
        match self {
            SecretDelta::FolderAdded(folder) => folder.zeroize(),
            SecretDelta::FolderUpdated { patch, .. } => patch.zeroize(),
            SecretDelta::EntryAdded(entry) => entry.zeroize(),
            SecretDelta::EntryUpdated { patch, .. } => patch.zeroize(),
            _ => {}
        }
    }
}
