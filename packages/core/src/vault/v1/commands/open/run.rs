use std::fs::File;
use std::path::Path;

use crate::crypto::encryption::factory::EncryptionAlgorithm;
use crate::crypto::kdf;
use crate::errors::{Error, Result};
use crate::vault::shared::commands::OpenConfig;
use crate::vault::v1::io::{self, IoContext};
use crate::vault::v1::keys::VaultKeys;
use crate::vault::v1::schema::vault::{Vault, VaultMeta};

pub fn run(source: &Path, password: &[u8], _: Option<OpenConfig>) -> Result<Vault> {
    let mut input = File::open(source).map_err(Error::Io)?;

    let header = io::header::read_header_at_top(&mut input)?;

    let master_key = kdf::derive_master_key(password, &header.salt)?;
    let metadata_key = VaultKeys::Metadata.derive(&master_key)?;

    let cipher = EncryptionAlgorithm::XChaCha20Poly1305.get()?;

    let mut vault = Vault {
        header,
        metadata: VaultMeta::default(),
    };

    let ctx = IoContext {
        cipher: cipher.as_ref(),
        compressor: None,
    };

    io::metadata::read_metadata(&mut vault, &mut input, metadata_key.as_ref(), &ctx)?;

    Ok(vault)
}
