use serde::{Deserialize, Serialize};

use openvault_crypto::keys::Salt;

pub const VAULT_MAGIC: &[u8; 6] = b"OPENV0";
pub const VAULT_MAGIC_LEN: usize = VAULT_MAGIC.len();
pub const VAULT_HEADER_SIZE: usize = 39;
pub const CRC_SIZE: usize = 4;
pub const PAYLOAD_SIZE: usize = VAULT_HEADER_SIZE - CRC_SIZE;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultHeader {
    pub magic: [u8; VAULT_MAGIC_LEN],
    pub version: u8,
    pub salt: Salt,
    pub metadata_offset: u64,
    pub metadata_size: u32,
    #[serde(skip)]
    pub crc: u32,
}

impl Default for VaultHeader {
    fn default() -> Self {
        Self {
            magic: *VAULT_MAGIC,
            version: 1,
            salt: Salt::default(),
            metadata_offset: 0,
            metadata_size: 0,
            crc: 0,
        }
    }
}
