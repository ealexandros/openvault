use serde::{Deserialize, Serialize};

pub const VAULT_INDEX_SIZE: usize = 18;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultIndex {
    pub snapshot_offset: u64,
    pub delta_offset: u64,
    pub delta_count: u16,
}

impl VaultIndex {
    pub fn new(snapshot_offset: u64, delta_offset: u64, delta_count: u16) -> Self {
        Self {
            snapshot_offset,
            delta_offset,
            delta_count,
        }
    }
}
