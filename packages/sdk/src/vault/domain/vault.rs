use chrono::{DateTime, Utc};
use std::path::PathBuf;
use uuid::Uuid;

use crate::domain::models::{filesystem::FileSystem, note::Note};

use std::fmt::Debug;

#[derive(Debug)]
pub struct Vault {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub version: u16,

    pub filesystem: FileSystem,
    pub notes: Vec<Note>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Vault {
    pub fn new(path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_string(),
            path,
            version: 0,
            filesystem: FileSystem::new(),
            notes: Vec::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

// impl Vault<Locked> {
//     /// Creates a new Vault representation in the Locked state.
//     /// This is typically used when first loading the vault header.
//     pub fn new_locked(id: Uuid, name: String, version: u16, path: PathBuf) -> Self {
//         Self {
//             id,
//             name,
//             path,
//             version,
//             // In locked state, we don't have access to contents yet
//             root: Folder {
//                 id: Uuid::nil(),
//                 name: "/".to_string(),
//                 parent_id: None,
//                 children: vec![],
//                 created_at: Utc::now(),
//                 updated_at: Utc::now(),
//             },
//             notes: vec![],
//             secrets: vec![],
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//             state: PhantomData,
//         }
//     }

//     /// Transitions to Unlocked state.
//     /// This method consumes the Locked vault and returns an Unlocked one.
//     /// In a real scenario, this would involve decryption.
//     pub fn unlock(
//         self,
//         decrypted_root: Folder,
//         notes: Vec<Note>,
//         secrets: Vec<Secret>,
//     ) -> Vault<Unlocked> {
//         Vault {
//             id: self.id,
//             name: self.name,
//             path: self.path,
//             version: self.version,
//             root: decrypted_root,
//             notes,
//             secrets,
//             created_at: self.created_at,
//             updated_at: self.updated_at,
//             state: PhantomData,
//         }
//     }
// }

// impl Vault<Unlocked> {
//     /// Locks the vault again, clearing sensitive data from memory.
//     pub fn lock(self) -> Vault<Locked> {
//         Vault::new_locked(self.id, self.name, self.version, self.path)
//     }

//     /// Example operation only available in Unlocked state
//     pub fn add_secret(&mut self, secret: Secret) {
//         self.secrets.push(secret);
//     }
// }
