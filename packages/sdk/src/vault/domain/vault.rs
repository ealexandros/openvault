use chrono::{DateTime, Utc};
use std::marker::PhantomData;
use std::path::PathBuf;
use uuid::Uuid;

use crate::domain::models::{folder::Folder, note::Note, secret::Secret};
use crate::domain::{Locked, Unlocked, VaultState};

/// The main entry point for the Vault domain.
///
/// It uses the Type State pattern to enforce security at compile time.
/// A `Vault<Locked>` cannot access sensitive data.
/// A `Vault<Unlocked>` has been decrypted and can perform sensitive operations.
#[derive(Debug)]
pub struct Vault<S: VaultState = Locked> {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,

    // Core data structures
    pub root: Folder,
    pub notes: Vec<Note>,
    pub secrets: Vec<Secret>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // PhantomData to hold the state
    pub state: PhantomData<S>,
}

impl Vault<Locked> {
    /// Creates a new Vault representation in the Locked state.
    /// This is typically used when first loading the vault header.
    pub fn new_locked(id: Uuid, name: String, path: PathBuf) -> Self {
        Self {
            id,
            name,
            path,
            // In locked state, we don't have access to contents yet
            root: Folder {
                id: Uuid::nil(),
                name: "/".to_string(),
                parent_id: None,
                children: vec![],
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            notes: vec![],
            secrets: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            state: PhantomData,
        }
    }

    /// Transitions to Unlocked state.
    /// This method consumes the Locked vault and returns an Unlocked one.
    /// In a real scenario, this would involve decryption.
    pub fn unlock(
        self,
        decrypted_root: Folder,
        notes: Vec<Note>,
        secrets: Vec<Secret>,
    ) -> Vault<Unlocked> {
        Vault {
            id: self.id,
            name: self.name,
            path: self.path,
            root: decrypted_root,
            notes,
            secrets,
            created_at: self.created_at,
            updated_at: self.updated_at,
            state: PhantomData,
        }
    }
}

impl Vault<Unlocked> {
    /// Locks the vault again, clearing sensitive data from memory.
    pub fn lock(self) -> Vault<Locked> {
        Vault::new_locked(self.id, self.name, self.path)
    }

    /// Example operation only available in Unlocked state
    pub fn add_secret(&mut self, secret: Secret) {
        self.secrets.push(secret);
    }
}
