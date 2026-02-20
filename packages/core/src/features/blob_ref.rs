use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlobRef {
    pub id: [u8; 32],
    pub size_bytes: u64,
    pub manifest_offset: u64,
}
