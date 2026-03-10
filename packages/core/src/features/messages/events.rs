use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

use super::error::MessagesError;
use super::models::{MessageContact, MessageCredentials};
use super::patch::MessageContactPatch;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct MessagesSnapshot {
    pub credentials: Option<MessageCredentials>,
    pub contacts: HashMap<Uuid, MessageContact>,
}

impl MessagesSnapshot {
    pub fn new(
        credentials: Option<MessageCredentials>,
        contacts: HashMap<Uuid, MessageContact>,
    ) -> Self {
        Self {
            credentials,
            contacts,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessagesDelta {
    ContactAdded(MessageContact),
    ContactUpdated {
        id: Uuid,
        patch: MessageContactPatch,
    },
    ContactDeleted(Uuid),
    CredentialsSet(MessageCredentials),
    CredentialsCleared,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessagesChange {
    Snapshot(MessagesSnapshot),
    Deltas(Vec<MessagesDelta>),
}

impl From<MessagesSnapshot> for MessagesChange {
    fn from(value: MessagesSnapshot) -> Self {
        Self::Snapshot(value)
    }
}

impl TryFrom<MessagesChange> for MessagesSnapshot {
    type Error = MessagesError;

    fn try_from(value: MessagesChange) -> Result<Self, Self::Error> {
        match value {
            MessagesChange::Snapshot(snapshot) => Ok(snapshot),
            MessagesChange::Deltas(_) => Err(MessagesError::InvalidSnapshot),
        }
    }
}

impl Zeroize for MessagesDelta {
    fn zeroize(&mut self) {
        match self {
            MessagesDelta::ContactAdded(contact) => contact.zeroize(),
            MessagesDelta::ContactUpdated { patch, .. } => {
                if let Some(name) = &mut patch.name {
                    name.zeroize();
                }
                if let Some(signing_public_key) = &mut patch.signing_pub_key {
                    signing_public_key.zeroize();
                }
                if let Some(ephemeral_public_key) = &mut patch.ephemeral_pub_key {
                    ephemeral_public_key.zeroize();
                }
            }
            MessagesDelta::CredentialsSet(credentials) => credentials.zeroize(),
            _ => {}
        }
    }
}
