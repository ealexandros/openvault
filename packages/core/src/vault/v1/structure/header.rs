use crate::constants;
use crate::errors::{Error, Result};
use crate::utils::io::ReadExt;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc32fast::Hasher;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub const VAULT_HEADER_SIZE: u64 = 67;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultHeader {
    pub magic: [u8; constants::VAULT_MAGIC_LEN],
    pub version: u8,
    pub salt: [u8; constants::SALT_LEN],
    pub index_offset: u64,
    pub files_offset: u64,
    pub notes_offset: u64,
    pub logs_offset: u64,
    pub decoy_offset: u64,
    pub crc: u32,
}

impl Default for VaultHeader {
    fn default() -> Self {
        Self {
            magic: *constants::VAULT_MAGIC,
            version: 1,
            salt: [0u8; constants::SALT_LEN],
            index_offset: 0,
            files_offset: 0,
            notes_offset: 0,
            logs_offset: 0,
            decoy_offset: 0,
            crc: 0,
        }
    }
}

impl VaultHeader {
    pub fn read_from_stream(reader: &mut impl Read) -> Result<Self> {
        let magic = reader.read_exact_array::<{ constants::VAULT_MAGIC_LEN }>()?;

        if &magic != constants::VAULT_MAGIC {
            return Err(Error::InvalidVaultMagic);
        }

        let version = reader.read_u8()?;
        let salt = reader.read_exact_array::<{ constants::SALT_LEN }>()?;

        let index_offset = reader.read_u64::<LittleEndian>()?;
        let files_offset = reader.read_u64::<LittleEndian>()?;
        let notes_offset = reader.read_u64::<LittleEndian>()?;
        let logs_offset = reader.read_u64::<LittleEndian>()?;
        let decoy_offset = reader.read_u64::<LittleEndian>()?;

        let mut hasher = Hasher::new();
        hasher.update(&magic);
        hasher.update(&[version]);
        hasher.update(&salt);
        hasher.update(&index_offset.to_le_bytes());
        hasher.update(&files_offset.to_le_bytes());
        hasher.update(&notes_offset.to_le_bytes());
        hasher.update(&logs_offset.to_le_bytes());
        hasher.update(&decoy_offset.to_le_bytes());

        let crc = reader.read_u32::<LittleEndian>()?;
        let expected_crc = hasher.finalize();

        if crc != expected_crc {
            return Err(Error::InvalidVaultChecksum);
        }

        Ok(Self {
            magic,
            version,
            salt,
            index_offset,
            files_offset,
            notes_offset,
            logs_offset,
            decoy_offset,
            crc,
        })
    }

    pub fn write_to_stream(&mut self, writer: &mut impl Write) -> Result<()> {
        let mut buf = Vec::with_capacity(VAULT_HEADER_SIZE as usize);

        buf.extend_from_slice(&self.magic);
        buf.push(self.version);
        buf.extend_from_slice(&self.salt);
        buf.write_u64::<LittleEndian>(self.index_offset)?;
        buf.write_u64::<LittleEndian>(self.files_offset)?;
        buf.write_u64::<LittleEndian>(self.notes_offset)?;
        buf.write_u64::<LittleEndian>(self.logs_offset)?;
        buf.write_u64::<LittleEndian>(self.decoy_offset)?;

        self.crc = crc32fast::hash(&buf);

        writer.write_all(&buf)?;
        writer.write_u32::<LittleEndian>(self.crc)?;

        Ok(())
    }
}
