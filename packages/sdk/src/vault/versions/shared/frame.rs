use std::io::Read;
use std::io::Write;

use openvault_crypto::encryption::NONCE_SIZE;

use crate::errors::Error;
use crate::errors::Result;

pub const ENCRYPTED_FILE_HEADER_SIZE: usize = 4 + NONCE_SIZE;

type Nonce = [u8; NONCE_SIZE];

pub struct FrameHeader {
    pub size: u32,
    pub nonce: Nonce,
}

impl FrameHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ENCRYPTED_FILE_HEADER_SIZE);
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(&self.nonce);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != ENCRYPTED_FILE_HEADER_SIZE {
            return Err(Error::InvalidEntryHeader);
        }

        let size_bytes: [u8; 4] = bytes
            .get(..4)
            .ok_or(Error::InvalidEntryHeader)?
            .try_into()
            .map_err(|_| Error::InvalidEntryHeader)?;

        let nonce: [u8; NONCE_SIZE] = bytes
            .get(4..4 + NONCE_SIZE)
            .ok_or(Error::InvalidEntryHeader)?
            .try_into()
            .map_err(|_| Error::InvalidEntryHeader)?;

        let size = u32::from_le_bytes(size_bytes);

        Ok(Self { size, nonce })
    }

    pub fn read_from(reader: &mut dyn Read) -> Result<Self> {
        let mut buffer = [0u8; ENCRYPTED_FILE_HEADER_SIZE];
        reader.read_exact(&mut buffer)?;
        Self::from_bytes(&buffer)
    }

    pub fn write_to(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write_all(&self.to_bytes())?;
        Ok(())
    }
}

pub fn read_frame(reader: &mut dyn Read) -> Result<(FrameHeader, Vec<u8>)> {
    let header = FrameHeader::read_from(reader)?;
    let mut payload = vec![0u8; header.size as usize];
    reader.read_exact(&mut payload)?;
    Ok((header, payload))
}

pub fn write_frame<W: Write>(writer: &mut W, nonce: &Nonce, ciphertext: &[u8]) -> Result {
    let header = FrameHeader {
        size: ciphertext.len() as u32,
        nonce: *nonce,
    };

    header.write_to(writer)?;
    writer.write_all(ciphertext)?;

    Ok(())
}
