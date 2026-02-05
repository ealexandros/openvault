use crate::constants::KEY_LEN;
use crate::crypto::{compression::Compressor, encryption::Cipher};
use crate::errors::Result;
use crate::vault::v1::structure::{FileEntry, FolderEntry, IndexEntry, VAULT_HEADER_SIZE, Vault};
use byteorder::{LittleEndian, WriteBytesExt};
use rand::RngCore;
use std::fs::File;
use std::io::{self, Seek, Write};
use std::path::Path;
use zeroize::Zeroizing;

pub struct VaultWriter<'a> {
    pub key: &'a Zeroizing<[u8; KEY_LEN]>,
    pub source: &'a Path,
    pub output: &'a mut File,
    pub compressor: Box<dyn Compressor>,
    pub cipher: Box<dyn Cipher>,
}

impl<'a> VaultWriter<'a> {
    pub fn write_file(&mut self, entry: &mut FileEntry) -> Result<()> {
        let full_path = self.source.join(&entry.path);
        let mut source_file = File::open(full_path)?;

        rand::rng().fill_bytes(&mut entry.nonce);

        let start_pos = self.output.stream_position()?;

        let mut buffer = Vec::new();
        self.compressor
            .compress_stream(&mut source_file, &mut buffer)?;

        self.cipher
            .encrypt_stream(self.key.as_ref(), &mut &buffer[..], self.output)?;

        let end_pos = self.output.stream_position()?;
        entry.offset = start_pos - VAULT_HEADER_SIZE;
        entry.compressed_size = end_pos - start_pos;

        Ok(())
    }
}

impl<'a> VaultWriter<'a> {
    pub fn finalize(
        &mut self,
        vault: &mut Vault,
        files: Vec<FileEntry>,
        folders: Vec<FolderEntry>,
    ) -> Result<()> {
        let index_pos = self.output.stream_position()?;

        vault.header.files_offset = VAULT_HEADER_SIZE;
        vault.header.index_offset = index_pos;
        vault.files_index.entries = files.into_iter().map(IndexEntry::File).collect();
        vault.folder_index.entries = folders;

        let indices = (
            &vault.files_index,
            &vault.notes_index,
            &vault.logs_index,
            &vault.folder_index,
        );

        let encoded = postcard::to_allocvec(&indices).map_err(|_| io::Error::other("Ser error"))?;

        let (enc_data, enc_meta) = self.cipher.encrypt(self.key.as_ref(), &encoded)?;

        self.output
            .write_u32::<LittleEndian>(enc_data.len() as u32)?;
        self.output
            .write_u32::<LittleEndian>(enc_meta.len() as u32)?;
        self.output.write_all(&enc_data)?;
        self.output.write_all(&enc_meta)?;

        self.output.rewind()?;
        vault.header.write_to_stream(self.output)?;

        Ok(())
    }
}
