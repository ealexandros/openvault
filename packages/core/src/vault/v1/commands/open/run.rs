use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use crate::crypto::encryption::factory::EncryptionAlgorithm;
use crate::crypto::kdf;
use crate::errors::{Error, Result};
use crate::utils::io::ReadExt;
use crate::vault::shared::commands::OpenConfig;
use crate::vault::v1::keys::VaultKeys;
use crate::vault::v1::schema::header::VaultHeader;
use crate::vault::v1::schema::vault::{Vault, VaultMeta};

pub fn run(source: &Path, password: &[u8], _: Option<OpenConfig>) -> Result<Vault> {
    let mut input = File::open(source).map_err(Error::Io)?;

    let header = VaultHeader::read_from_stream(&mut input)?;

    let master_key = kdf::derive_master_key(password, &header.salt)?;
    let metadata_key = VaultKeys::Metadata.derive(&master_key)?;

    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.get()?;

    input.seek(SeekFrom::Start(header.metadata_offset))?;
    let meta_data = input.read_exact_vec(header.metadata_size as usize)?;

    let decrypted_meta = cipher.decrypt(metadata_key.as_ref(), &meta_data)?;

    let metadata: VaultMeta =
        postcard::from_bytes(&decrypted_meta).map_err(|_| Error::InvalidVaultFormat)?;

    Ok(Vault { header, metadata })
}
