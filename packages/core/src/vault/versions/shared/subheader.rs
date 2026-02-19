use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use crate::errors::Result;

pub const SUBHEADER_SIZE: usize = size_of::<Subheader>();

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Subheader {
    pub checkpoint_offset: u64,
    pub tail_record_offset: u64,
    pub last_sequence: u64,
}

impl Subheader {
    pub fn new(checkpoint_offset: u64, tail_record_offset: u64) -> Self {
        Self {
            checkpoint_offset,
            tail_record_offset,
            last_sequence: 0,
        }
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let subheader = Self {
            checkpoint_offset: reader.read_u64::<LittleEndian>()?,
            tail_record_offset: reader.read_u64::<LittleEndian>()?,
            last_sequence: reader.read_u64::<LittleEndian>()?,
        };
        Ok(subheader)
    }

    pub fn write_to<W: Write>(self, writer: &mut W) -> Result {
        writer.write_u64::<LittleEndian>(self.checkpoint_offset)?;
        writer.write_u64::<LittleEndian>(self.tail_record_offset)?;
        writer.write_u64::<LittleEndian>(self.last_sequence)?;
        Ok(())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut reader = bytes;
        Subheader::read_from(&mut reader)
    }

    pub fn to_bytes(self) -> Result<[u8; SUBHEADER_SIZE]> {
        let mut buffer = [0u8; SUBHEADER_SIZE];
        let mut writer = &mut buffer[..];
        self.write_to(&mut writer)?;
        Ok(buffer)
    }
}
