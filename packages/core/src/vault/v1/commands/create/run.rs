use std::io::{Seek, SeekFrom};
use std::path::Path;
use zeroize::Zeroize;

use crate::crypto::kdf;
use crate::errors::Result;
use crate::utils::fs::{create_new_file, remove_file_if_exists};
use crate::vault::shared::commands::CreateConfig;
use crate::vault::v1::builder::VaultBuilder;
use crate::vault::v1::commands::create::scanner::scan_filesystem;
use crate::vault::v1::commands::create::writer::VaultWriter;
use crate::vault::v1::keys::VaultKeys;

pub fn run(source: &Path, password: &[u8], config: Option<CreateConfig>) -> Result {
    let config = config.unwrap_or_default();

    let (files, folders) = scan_filesystem(source)?;

    let salt = kdf::generate_default_salt();
    let mut master_key = kdf::derive_master_key(password, &salt)?;

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

    vault.header.write_to_stream(&mut output)?;

    let mut writer = VaultWriter::new(
        source,
        &output,
        config.cipher.get()?,
        config.compression.get()?,
    );

    writer.write_files(&mut vault, files_key.as_ref())?;
    writer.write_metadata(&mut vault, metadata_key.as_ref())?;

    output.seek(SeekFrom::Start(0))?;

    vault.header.write_to_stream(&mut output)?;

    master_key.zeroize();

    Ok(())
}
