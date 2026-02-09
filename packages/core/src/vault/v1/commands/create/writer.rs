use crate::crypto::{compression::Compressor, encryption::Cipher};
use crate::errors::{Error, Result};
use crate::vault::v1::schema::header::VAULT_HEADER_SIZE;
use crate::vault::v1::schema::vault::Vault;
use std::fs::File;
use std::io::{Seek, Write};
use std::path::Path;

pub struct VaultWriter<'a> {
    source: &'a Path,
    output: &'a File,
    cipher: Box<dyn Cipher>,
    compressor: Box<dyn Compressor>,
}

impl<'a> VaultWriter<'a> {
    pub fn new(
        source: &'a Path,
        output: &'a File,
        cipher: Box<dyn Cipher>,
        compressor: Box<dyn Compressor>,
    ) -> Self {
        Self {
            source,
            output,
            cipher,
            compressor,
        }
    }

    pub fn write_metadata(&mut self, vault: &mut Vault, key: &[u8]) -> Result {
        let pos = self.output.stream_position()?;

        let data = postcard::to_stdvec(&vault.metadata).map_err(|_| Error::InvalidVaultFormat)?;

        let (enc, nonce) = self.cipher.encrypt(key, &data)?;

        self.output.write_all(&enc)?;

        vault.header.metadata_offset = pos;
        vault.header.metadata_size = enc.len() as u32;
        vault.header.metadata_nonce = nonce.try_into().map_err(|_| Error::EncryptionFailed)?;

        Ok(())
    }

    pub fn write_files(&mut self, vault: &mut Vault, key: &[u8]) -> Result {
        let files = vault.metadata.filesystem.files.iter_mut();

        for file in files {
            let full_path = self.source.join(file.relative_path.as_path());
            let mut input = File::open(full_path)?;

            let start = self.output.stream_position()?;

            let mut buf = Vec::new();
            self.compressor.compress_stream(&mut input, &mut buf)?;

            self.cipher
                .encrypt_stream(key, &mut &buf[..], &mut self.output)?;

            let end = self.output.stream_position()?;

            file.blob.offset = start - VAULT_HEADER_SIZE;
            file.blob.size = end - start;
        }

        Ok(())
    }
}
