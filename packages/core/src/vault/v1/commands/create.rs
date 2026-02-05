mod scanner;
mod writer;

use crate::crypto::kdf;
use crate::errors::Result;
use crate::utils::fs;
use crate::vault::shared::commands::CreateConfig;
use crate::vault::v1::commands::create::scanner::scan_filesystem;
use crate::vault::v1::commands::create::writer::VaultWriter;
use crate::vault::v1::structure::{SectionIndex, VAULT_HEADER_SIZE, Vault, VaultHeader};
use std::io::{Seek, SeekFrom};
use std::path::Path;
use zeroize::Zeroize;

pub fn run(source_path: &Path, password: &[u8], config: Option<CreateConfig>) -> Result {
    let config = config.unwrap_or_default();

    let (mut files, folders) = scan_filesystem(source_path)?;

    let salt = kdf::generate_default_salt();
    let mut key = kdf::derive_master_key(password, &salt)?;

    let file_path = Path::new(&config.output_path).join(config.filename);

    fs::remove_file_if_exists(&file_path).unwrap();
    let mut output_file = fs::create_new_file(&file_path).unwrap();

    output_file.seek(SeekFrom::Start(VAULT_HEADER_SIZE))?;

    let mut vault = Vault {
        header: VaultHeader::default(),
        files_index: SectionIndex::default(),
        notes_index: SectionIndex::default(),
        logs_index: SectionIndex::default(),
        folder_index: SectionIndex::default(),
        decoy_vault: None,
    };

    vault.header.salt = salt;

    {
        let mut writer = VaultWriter {
            key: &mut key,
            source: source_path,
            output: &mut output_file,
            cipher: config.cipher.get()?,
            compressor: config.compression.get()?,
        };

        for entry in files.iter_mut() {
            writer.write_file(entry)?;
        }

        writer.finalize(&mut vault, files, folders)?;
    }

    key.zeroize();

    Ok(())
}
