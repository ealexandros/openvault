use std::path::Path;

use openvault_crypto::keys::salt::Salt;

use crate::errors::Result;
use crate::internal::fs::{create_new_file, open_with_read_write, remove_if_exists};
use crate::operations::config::CreateConfig;
use crate::vault::boot_header::BootHeader;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::resolve_format;

pub fn create_vault_with(path: &Path, password: &[u8], config: CreateConfig) -> Result {
    let salt = Salt::random();
    let keyring = Keyring::derive(password, &salt)?;
    let boot_header = BootHeader::new(salt, Some(config.version));

    if config.overwrite {
        remove_if_exists(path)?;
    }
    let mut file = create_new_file(path)?;

    boot_header.write_to(&mut file)?;

    let engine = resolve_format(config.version)?;
    engine.init_layout(&mut file, &keyring)?;

    Ok(())
}

pub fn create_vault(path: &Path, password: &[u8]) -> Result {
    create_vault_with(path, password, CreateConfig::default())
}

pub fn open_vault(path: &Path, password: &[u8]) -> Result<VaultSession> {
    let mut file = open_with_read_write(path)?;

    let boot_header = BootHeader::read_from(&mut file)?;
    let keyring = Keyring::derive(password, &Salt::from(boot_header.salt))?;

    let engine = resolve_format(boot_header.version)?;

    engine.read_subheader(&mut file, &keyring)?;

    Ok(VaultSession::new(
        file,
        boot_header.version,
        keyring,
        engine,
    ))
}

pub fn create_and_open_vault(
    path: &Path,
    password: &[u8],
    config: CreateConfig,
) -> Result<VaultSession> {
    create_vault_with(path, password, config)?;
    open_vault(path, password)
}

pub fn change_password() {
    todo!()
}
