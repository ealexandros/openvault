use crate::constants;
use crate::errors::{Error, Result};
use crate::utils::io::ReadExt;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub const VAULT_HEADER_SIZE: u64 = 63;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultHeader {
    pub magic: [u8; constants::VAULT_MAGIC_LEN],
    pub version: u8,
    pub salt: [u8; constants::SALT_LEN],
    pub metadata_offset: u64,
    pub metadata_size: u32,
    pub metadata_nonce: [u8; constants::NONCE_LEN],
    pub crc: u32,
}

impl Default for VaultHeader {
    fn default() -> Self {
        Self {
            magic: *constants::VAULT_MAGIC,
            version: 1,
            salt: [0u8; constants::SALT_LEN],
            metadata_offset: 0,
            metadata_size: 0,
            metadata_nonce: [0u8; constants::NONCE_LEN],
            crc: 0,
        }
    }
}

impl VaultHeader {
    pub fn read_from_stream(reader: &mut impl Read) -> Result<Self> {
        let buf = reader.read_exact_array::<{ VAULT_HEADER_SIZE as usize }>()?;

        let (data, crc_bytes) = buf.split_at(buf.len() - 4);
        let crc = u32::from_le_bytes(crc_bytes.try_into().unwrap());

        let expected_crc = crc32fast::hash(data);

        if crc != expected_crc {
            return Err(Error::InvalidVaultChecksum);
        }

        let header: VaultHeader =
            postcard::from_bytes(data).map_err(|_| Error::InvalidVaultFormat)?;

        Ok(header)
    }

    pub fn write_to_stream(&mut self, writer: &mut impl Write) -> Result<()> {
        self.crc = 0;
        let mut buf = postcard::to_stdvec(self).map_err(|_| Error::InvalidVaultFormat)?;

        let target_size = VAULT_HEADER_SIZE as usize - 4;

        if buf.len() > target_size {
            return Err(Error::InvalidVaultFormat);
        }

        buf.resize(target_size, 0);

        let crc = crc32fast::hash(&buf);
        buf.extend_from_slice(&crc.to_le_bytes());

        writer.write_all(&buf)?;
        self.crc = crc;

        Ok(())
    }
}
