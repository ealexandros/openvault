use serde::{Deserialize, Serialize};

use crate::versions::v1::schema::entries::{FileSystemMeta, LogMeta, NoteMeta, SecretMeta};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VaultMeta {
    pub filesystem: FileSystemMeta,
    pub notes: Vec<NoteMeta>,
    pub logs: Vec<LogMeta>,
    pub secrets: Vec<SecretMeta>,
    pub decoy: Option<Box<VaultMeta>>,
}
