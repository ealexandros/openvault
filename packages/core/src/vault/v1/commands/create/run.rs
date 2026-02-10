use std::io::{Seek, SeekFrom};
use std::path::Path;

use crate::crypto::kdf;
use crate::errors::Result;
use crate::utils::fs::{create_new_file, remove_file_if_exists};
use crate::vault::shared::commands::CreateConfig;
use crate::vault::v1::builder::VaultBuilder;
use crate::vault::v1::io::{self, IoContext};
use crate::vault::v1::keys::VaultKeys;
use crate::vault::v1::schema::header::VAULT_HEADER_SIZE;

pub fn run(source: &Path, password: &[u8], config: Option<CreateConfig>) -> Result {
    let config = config.unwrap_or_default();

    let (files, folders) = io::scanner::scan_filesystem(source)?;

    let salt = kdf::generate_default_salt();
    let master_key = kdf::derive_master_key(password, &salt)?;

    let metadata_key = VaultKeys::Metadata.derive(&master_key)?;
    let files_key = VaultKeys::Files.derive(&master_key)?;

    let output_path = Path::new(&config.output_path).join(config.filename);

    if config.overwrite_existing {
        remove_file_if_exists(&output_path)?;
    }

    let mut output = create_new_file(&output_path)?;

    let mut vault = VaultBuilder::new(salt)
        .add_files(files)
        .add_folders(folders)
        .build();

    output.seek(SeekFrom::Start(VAULT_HEADER_SIZE as u64))?;

    let cipher = config.cipher.get()?;
    let compressor = config.compression.get()?;

    let ctx = IoContext {
        cipher: cipher.as_ref(),
        compressor: Some(compressor.as_ref()),
    };

    for file in vault.metadata.filesystem.files.iter_mut() {
        io::blob::write_file(file, source, &mut output, files_key.as_ref(), &ctx)?;
    }

    io::metadata::write_metadata(&mut vault, &mut output, metadata_key.as_ref(), &ctx)?;
    io::header::write_header_at_top(&mut vault.header, &mut output)?;

    Ok(())
}
