use std::io::{Read, Seek, SeekFrom, Write};

use crate::errors::{Error, Result};

pub const SUBHEADER_SIZE: usize = 16;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Subheader {
    pub checkpoint_offset: u64,
    pub delta_offset: u64,
}

impl Subheader {
    pub fn new(checkpoint_offset: u64, delta_offset: u64) -> Self {
        Self {
            checkpoint_offset,
            delta_offset,
        }
    }

    pub fn to_bytes(&self) -> [u8; SUBHEADER_SIZE] {
        let mut bytes = [0u8; SUBHEADER_SIZE];
        bytes[..8].copy_from_slice(&self.checkpoint_offset.to_le_bytes());
        bytes[8..].copy_from_slice(&self.delta_offset.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != SUBHEADER_SIZE {
            return Err(Error::InvalidVaultFormat);
        }

        let checkpoint_bytes = bytes[..8]
            .try_into()
            .map_err(|_| Error::InvalidVaultFormat)?;
        let delta_bytes = bytes[8..]
            .try_into()
            .map_err(|_| Error::InvalidVaultFormat)?;

        let checkpoint_offset = u64::from_le_bytes(checkpoint_bytes);
        let delta_offset = u64::from_le_bytes(delta_bytes);

        Ok(Self {
            checkpoint_offset,
            delta_offset,
        })
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; SUBHEADER_SIZE];
        reader.read_exact(&mut buffer)?;
        Self::from_bytes(&buffer)
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result {
        writer.write_all(&self.to_bytes()).map_err(Into::into)
    }
}

pub fn read_subheader_at<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Subheader> {
    reader.seek(SeekFrom::Start(offset))?;
    Subheader::read_from(reader)
}

pub fn write_subheader_at<W: Write + Seek>(
    writer: &mut W,
    offset: u64,
    subheader: &Subheader,
) -> Result {
    writer.seek(SeekFrom::Start(offset))?;
    subheader.write_to(writer)
}
