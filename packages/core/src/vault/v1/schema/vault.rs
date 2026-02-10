use serde::{Deserialize, Serialize};

use crate::vault::v1::schema::entries::{FileSystemMeta, LogMeta, NoteMeta, SecretMeta};
use crate::vault::v1::schema::header::VaultHeader;

/// VaultMeta contains all metadata for vault contents.
/// Serialized as a single encrypted block at EOF.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VaultMeta {
    pub filesystem: FileSystemMeta,
    pub notes: Vec<NoteMeta>,
    pub logs: Vec<LogMeta>,
    pub secrets: Vec<SecretMeta>,
    pub decoy: Option<Box<VaultMeta>>,
}

/// Vault is the top-level structure representing an open vault in memory.
/// It combines the file header and the decrypted metadata.
#[derive(Clone, Debug, Default)]
pub struct Vault {
    pub header: VaultHeader,
    pub metadata: VaultMeta,
}
