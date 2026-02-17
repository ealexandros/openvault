use std::io::{Read, Seek, Write};

use openvault_crypto::keys::MasterKey;

use crate::domain::Vault;
use crate::errors::Result;

pub trait ReadSeek: Read + Seek {}

impl<T: Read + Seek> ReadSeek for T {}

pub trait WriteSeek: Write + Seek {}

impl<T: Write + Seek> WriteSeek for T {}

pub trait VersionHandler {
    fn read(&self, reader: &mut dyn ReadSeek, key: &MasterKey) -> Result<Vault>;
    fn add_delta(&self, writer: &mut dyn WriteSeek, data: &[u8], key: &MasterKey) -> Result;
    fn add_snapshot(&self, writer: &mut dyn WriteSeek, vault: &Vault, key: &MasterKey) -> Result;
}
