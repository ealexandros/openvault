use std::io::{Read, Write};

use openvault_crypto::encryption::{NONCE_SIZE, Nonce};

use crate::errors::{Error, Result};

pub const FRAME_HEADER_SIZE: usize = 4 + NONCE_SIZE;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameHeader {
    pub size: u32,
    pub nonce: Nonce,
}

// @todo-soon rethink about this code style..

impl FrameHeader {
    pub fn new(size: u32, nonce: Nonce) -> Self {
        Self { size, nonce }
    }

    pub fn to_bytes(&self) -> [u8; FRAME_HEADER_SIZE] {
        let mut bytes = [0u8; FRAME_HEADER_SIZE];
        bytes[..4].copy_from_slice(&self.size.to_le_bytes());
        bytes[4..].copy_from_slice(self.nonce.as_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != FRAME_HEADER_SIZE {
            return Err(Error::InvalidVaultFormat);
        }

        let size_bytes = bytes[..4]
            .try_into()
            .map_err(|_| Error::InvalidVaultFormat)?;
        let nonce_bytes = bytes[4..]
            .try_into()
            .map_err(|_| Error::InvalidVaultFormat)?;

        Ok(Self {
            size: u32::from_le_bytes(size_bytes),
            nonce: Nonce::new(nonce_bytes),
        })
    }

    pub fn read_from<R: Read + ?Sized>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; FRAME_HEADER_SIZE];
        reader.read_exact(&mut buffer)?;
        Self::from_bytes(&buffer)
    }

    pub fn write_to<W: Write + ?Sized>(&self, writer: &mut W) -> Result {
        writer.write_all(&self.to_bytes())?;
        Ok(())
    }
}

pub fn read_frame_payload<R: Read + ?Sized>(reader: &mut R, size: u32) -> Result<Vec<u8>> {
    let mut payload = vec![0u8; size as usize];
    reader.read_exact(&mut payload)?;
    Ok(payload)
}

pub fn read_frame<R: Read + ?Sized>(reader: &mut R) -> Result<(FrameHeader, Vec<u8>)> {
    let header = FrameHeader::read_from(reader)?;
    let payload = read_frame_payload(reader, header.size)?;
    Ok((header, payload))
}

pub fn write_frame<W: Write + ?Sized>(writer: &mut W, nonce: &Nonce, ciphertext: &[u8]) -> Result {
    let header = FrameHeader::new(ciphertext.len() as u32, *nonce);
    header.write_to(writer)?;
    writer.write_all(ciphertext)?;
    Ok(())
}
