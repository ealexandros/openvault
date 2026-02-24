use std::io::{Cursor, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc32fast;
use openvault_crypto::compression::CompressionAlgorithm;
use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::salt::{SALT_SIZE, Salt};

use crate::errors::{Error, Result};
use crate::internal::io_ext::{ReadExt, Reader, SeekExt, Writer};
use crate::vault::versions::factory::LATEST_FORMAT_VERSION;

const VAULT_MAGIC: &[u8; 6] = b"OPENV0";
const VAULT_MAGIC_SIZE: usize = VAULT_MAGIC.len();
const CRC_SIZE: usize = 4;

const VAULT_PAYLOAD_SIZE: usize = 26;

#[derive(Clone, Debug)]
pub struct BootHeader {
    pub magic: [u8; VAULT_MAGIC_SIZE],
    pub version: u16,
    pub salt: [u8; SALT_SIZE],
    pub cipher: EncryptionAlgorithm,
    pub compressor: CompressionAlgorithm,
}

impl BootHeader {
    pub const SIZE: usize = VAULT_PAYLOAD_SIZE + CRC_SIZE;

    pub fn new(
        salt: Salt,
        version: Option<u16>,
        cipher: Option<EncryptionAlgorithm>,
        compressor: Option<CompressionAlgorithm>,
    ) -> Self {
        Self {
            magic: *VAULT_MAGIC,
            version: version.unwrap_or(LATEST_FORMAT_VERSION),
            salt: salt.into_bytes(),
            cipher: cipher.unwrap_or(EncryptionAlgorithm::XChaCha20Poly1305),
            compressor: compressor.unwrap_or(CompressionAlgorithm::Zstd),
        }
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        let mut bytes = [0u8; Self::SIZE];

        {
            let mut cursor = Cursor::new(&mut bytes[..VAULT_PAYLOAD_SIZE]);

            cursor.write_all(&self.magic)?;
            cursor.write_all(&self.version.to_le_bytes())?;
            cursor.write_all(&self.salt)?;
            cursor.write_u8(self.cipher as u8)?;
            cursor.write_u8(self.compressor as u8)?;
        }

        let crc = crc32fast::hash(&bytes[..VAULT_PAYLOAD_SIZE]);
        bytes[VAULT_PAYLOAD_SIZE..].copy_from_slice(&crc.to_le_bytes());

        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != Self::SIZE {
            return Err(Error::InvalidVaultFormat);
        }

        let (payload, crc_bytes) = bytes.split_at(VAULT_PAYLOAD_SIZE);

        let stored_crc = u32::from_le_bytes(
            crc_bytes
                .try_into()
                .map_err(|_| Error::InvalidVaultFormat)?,
        );
        let computed_crc = crc32fast::hash(payload);

        if stored_crc != computed_crc {
            return Err(Error::InvalidVaultChecksum);
        }

        let mut cursor = Cursor::new(payload);

        let magic = cursor.read_exact_arr::<VAULT_MAGIC_SIZE>()?;

        if magic != *VAULT_MAGIC {
            return Err(Error::InvalidVaultFormat);
        }

        let version = cursor.read_u16::<LittleEndian>()?;
        let salt = cursor.read_exact_arr::<SALT_SIZE>()?;
        let cipher = cursor.read_u8()?;
        let compressor = cursor.read_u8()?;

        Ok(Self {
            magic,
            version,
            salt,
            cipher: EncryptionAlgorithm::try_from(cipher)?,
            compressor: CompressionAlgorithm::try_from(compressor)?,
        })
    }

    pub fn read_from(reader: &mut Reader) -> Result<Self> {
        reader.seek_to_start()?;
        let buffer = reader.read_exact_arr::<{ Self::SIZE }>()?;
        Self::from_bytes(&buffer)
    }

    pub fn write_to(&self, writer: &mut Writer) -> Result {
        writer.seek_to_start()?;
        writer.write_all(&self.to_bytes()?)?;
        Ok(())
    }
}
