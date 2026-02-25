use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlobRef {
    pub id: Uuid,
    pub size_bytes: u64,
    pub manifest_offset: u64,
}

impl BlobRef {
    pub fn new(id: Uuid, size_bytes: u64, manifest_offset: u64) -> Self {
        Self {
            id,
            size_bytes,
            manifest_offset,
        }
    }
}
