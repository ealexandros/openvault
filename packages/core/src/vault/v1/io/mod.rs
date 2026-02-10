pub mod blob;
pub mod header;
pub mod metadata;
pub mod scanner;

use crate::crypto::compression::Compressor;
use crate::crypto::encryption::Cipher;
use crate::errors::{Error, Result};

pub struct IoContext<'a> {
    pub cipher: &'a dyn Cipher,
    pub compressor: Option<&'a dyn Compressor>,
}

impl IoContext<'_> {
    pub fn compressor(&self) -> Result<&dyn Compressor> {
        self.compressor.ok_or(Error::InvalidVaultFormat)
    }
}
