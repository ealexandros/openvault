use argon2::password_hash::rand_core::{OsRng, RngCore};
use std::io::Read;

use crate::errors::{Error, Result};

pub const NONCE_SIZE: usize = 24;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Nonce([u8; NONCE_SIZE]);

impl Nonce {
    pub fn new(bytes: [u8; NONCE_SIZE]) -> Self {
        Self(bytes)
    }

    pub fn read_from<R: Read + ?Sized>(reader: &mut R) -> Result<Self> {
        let mut bytes = [0u8; NONCE_SIZE];
        reader.read_exact(&mut bytes)?;
        Ok(Self(bytes))
    }

    pub fn random() -> Self {
        let mut bytes = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; NONCE_SIZE] {
        &self.0
    }

    pub fn into_bytes(self) -> [u8; NONCE_SIZE] {
        self.0
    }

    pub fn parse_prefixed(input: &[u8]) -> Result<(Self, &[u8])> {
        if input.len() < NONCE_SIZE {
            return Err(Error::DecryptionFailed);
        }

        let mut nonce = [0u8; NONCE_SIZE];
        nonce.copy_from_slice(&input[..NONCE_SIZE]);

        Ok((Self(nonce), &input[NONCE_SIZE..]))
    }

    pub fn increment(&mut self) {
        for byte in self.0.iter_mut().rev() {
            let (new, overflow) = byte.overflowing_add(1);

            *byte = new;

            if !overflow {
                break;
            }
        }
    }
}
