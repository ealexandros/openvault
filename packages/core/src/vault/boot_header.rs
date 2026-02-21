use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};

use openvault_crypto::keys::salt::{SALT_SIZE, Salt};

use crate::errors::{Error, Result};
use crate::internal::hashing;
use crate::internal::io_ext::SeekExt;
use crate::vault::versions::factory::LATEST_VERSION;

pub const VAULT_MAGIC: &[u8; 6] = b"OPENV0";
pub const VAULT_MAGIC_SIZE: usize = VAULT_MAGIC.len();
pub const CRC_SIZE: usize = 4;

pub const VAULT_PAYLOAD_SIZE: usize = 24;
pub const VAULT_TOTAL_SIZE: usize = VAULT_PAYLOAD_SIZE + CRC_SIZE;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootHeader {
    pub magic: [u8; VAULT_MAGIC_SIZE],
    pub version: u16,
    pub salt: [u8; SALT_SIZE],
}

impl BootHeader {
    pub fn new(salt: Salt, version: Option<u16>) -> Self {
        Self {
            magic: *VAULT_MAGIC,
            version: version.unwrap_or(LATEST_VERSION),
            salt: salt.into_bytes(),
        }
    }

    pub fn to_bytes(&self) -> Result<[u8; VAULT_TOTAL_SIZE]> {
        let mut payload = postcard::to_stdvec(self).map_err(|_| Error::InvalidVaultFormat)?;

        if payload.len() > VAULT_PAYLOAD_SIZE {
            return Err(Error::InvalidVaultFormat);
        }

        payload.resize(VAULT_PAYLOAD_SIZE, 0);

        let crc = hashing::compute_crc(&payload);
        let mut out = [0u8; VAULT_TOTAL_SIZE];

        out[..VAULT_PAYLOAD_SIZE].copy_from_slice(&payload);
        out[VAULT_PAYLOAD_SIZE..].copy_from_slice(&crc.to_le_bytes());

        Ok(out)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != VAULT_TOTAL_SIZE {
            return Err(Error::InvalidVaultFormat);
        }

        let (payload, crc_bytes) = bytes.split_at(VAULT_PAYLOAD_SIZE);
        let stored_crc = u32::from_le_bytes(
            crc_bytes
                .try_into()
                .map_err(|_| Error::InvalidVaultFormat)?,
        );
        let computed_crc = hashing::compute_crc(payload);

        if stored_crc != computed_crc {
            return Err(Error::InvalidVaultChecksum);
        }

        let header: Self = postcard::from_bytes(payload).map_err(|_| Error::InvalidVaultFormat)?;

        if header.magic != *VAULT_MAGIC {
            return Err(Error::InvalidVaultFormat);
        }

        Ok(header)
    }

    pub fn read_from<R: Seek + Read>(reader: &mut R) -> Result<Self> {
        reader.seek_start()?;

        let mut buffer = [0u8; VAULT_TOTAL_SIZE];
        reader.read_exact(&mut buffer)?;
        Self::from_bytes(&buffer)
    }

    pub fn write_to<W: Seek + Write>(&self, writer: &mut W) -> Result {
        writer.seek_start()?;
        writer.write_all(&self.to_bytes()?)?;
        Ok(())
    }
}
