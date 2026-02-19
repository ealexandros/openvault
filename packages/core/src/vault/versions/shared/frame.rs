use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use openvault_crypto::encryption::Nonce;
use std::io::{Read, Write};

use crate::errors::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameHeader {
    pub size: u32,
    pub nonce: Nonce,
}

impl FrameHeader {
    pub const SIZE: usize = size_of::<FrameHeader>();

    pub fn new(size: u32, nonce: Nonce) -> Self {
        Self { size, nonce }
    }

    pub fn write_to<W: Write + ?Sized>(&self, writer: &mut W) -> Result {
        writer.write_u32::<LittleEndian>(self.size)?;
        writer.write_all(self.nonce.as_bytes())?;
        Ok(())
    }

    pub fn read_from<R: Read + ?Sized>(reader: &mut R) -> Result<Self> {
        let size = reader.read_u32::<LittleEndian>()?;
        let nonce = Nonce::read_from(reader)?;
        Ok(Self { size, nonce })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        let mut buffer = [0u8; Self::SIZE];
        let mut cursor = std::io::Cursor::new(&mut buffer[..]);
        self.write_to(&mut cursor)?;
        Ok(buffer)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != Self::SIZE {
            return Err(Error::InvalidVaultFormat);
        }

        let mut cursor = std::io::Cursor::new(bytes);
        Self::read_from(&mut cursor)
    }
}

pub fn read_frame<R: Read + ?Sized>(reader: &mut R) -> Result<(FrameHeader, Vec<u8>)> {
    let header = FrameHeader::read_from(reader)?;
    let mut payload = vec![0u8; header.size as usize];
    reader.read_exact(&mut payload)?;
    Ok((header, payload))
}

pub fn write_frame<W: Write + ?Sized>(writer: &mut W, nonce: &Nonce, payload: &[u8]) -> Result<()> {
    let header = FrameHeader::new(payload.len() as u32, *nonce);
    header.write_to(writer)?;
    writer.write_all(payload)?;
    Ok(())
}
