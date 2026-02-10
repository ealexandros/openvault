use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

use crate::errors::{Error, Result};
use crate::utils::io::ReadExt;
use crate::vault::v1::io::IoContext;
use crate::vault::v1::schema::vault::Vault;

pub fn write_metadata(vault: &mut Vault, file: &mut File, key: &[u8], ctx: &IoContext) -> Result {
    let offset = file.stream_position()?;

    let plaintext = postcard::to_stdvec(&vault.metadata).map_err(|_| Error::InvalidVaultFormat)?;
    let ciphertext = ctx.cipher.encrypt(key, &plaintext)?;

    file.write_all(&ciphertext)?;

    vault.header.metadata_offset = offset;
    vault.header.metadata_size = ciphertext.len() as u32;

    Ok(())
}

pub fn read_metadata(vault: &mut Vault, file: &mut File, key: &[u8], ctx: &IoContext) -> Result {
    file.seek(SeekFrom::Start(vault.header.metadata_offset))?;

    let ciphertext = file.read_exact_vec(vault.header.metadata_size as usize)?;
    let plaintext = ctx.cipher.decrypt(key, &ciphertext)?;

    vault.metadata = postcard::from_bytes(&plaintext).map_err(|_| Error::InvalidVaultFormat)?;

    Ok(())
}
