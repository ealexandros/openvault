use std::fs::File;
use std::path::Path;

use openvault_crypto::compression::factory::CompressionAlgorithm;
use openvault_crypto::encryption::factory::EncryptionAlgorithm;
use openvault_crypto::keys::MasterKey;

use crate::domain::Vault;
use crate::errors::{Error, Result};
use crate::operations::scanner;
use crate::versions::factory::{LATEST_VERSION, get_handler};
use crate::versions::shared::header::VaultHeader;

#[derive(Debug)]
pub struct CreateConfig {
    pub compression: CompressionAlgorithm,
    pub cipher: EncryptionAlgorithm,
    pub output_path: String,
    pub filename: String,
    pub overwrite_existing: bool,
}

pub fn create_vault(path: String, password: String, config: CreateConfig) -> Result<Vault> {
    let output_path = Path::new(&config.output_path).join(&config.filename);

    if output_path.exists() && !config.overwrite_existing {
        return Err(Error::file_exists());
    }

    let (_master_key, salt) = MasterKey::derive_with_random_salt(password.as_bytes())?;

    let (files, folders) = scanner::scan_filesystem(Path::new(&path))?;

    let mut vault = Vault::new(output_path.clone());

    vault.filesystem.add_files(files);
    vault.filesystem.add_folders(folders);

    println!("{:#?}", vault);

    let mut file = File::create(&output_path)?;

    let header = VaultHeader::new(salt, Some(LATEST_VERSION));
    header.write_to(&mut file)?;

    let _handler = get_handler(vault.version)?;

    Ok(vault)
}
