use openvault_crypto::keys::salt::Salt;
use std::path::Path;

use crate::VAULT_EXTENSION;
use crate::errors::{Error, Result};
use crate::internal::fs::{create_new_file, open_with_read_write, remove_if_exists, resolve_path};
use crate::operations::config::{CreateConfig, OpenConfig};
use crate::vault::boot_header::BootHeader;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::resolve_format;
use crate::vault::versions::shared::traits::FormatContext;

pub fn create_vault_with(path: &Path, password: &[u8], config: CreateConfig) -> Result {
    let path = resolve_path(path, &config.filename, VAULT_EXTENSION);

    let salt = Salt::random();
    let keyring = Keyring::derive(password, &salt)?;
    let boot_header = BootHeader::new(salt, Some(config.version));

    if config.overwrite {
        remove_if_exists(&path)?;
    }
    let mut file = create_new_file(&path)?;

    boot_header.write_to(&mut file)?;

    let format = resolve_format(config.version)?;
    let context = FormatContext::new(&keyring, config.compression, config.cipher);

    format.init_layout(&mut file, &context)?;

    Ok(())
}

pub fn create_vault(path: &Path, password: &[u8]) -> Result {
    create_vault_with(path, password, CreateConfig::default())
}

pub fn open_vault(path: &Path, password: &[u8], config: OpenConfig) -> Result<VaultSession> {
    let mut file = open_with_read_write(path)?;

    let boot_header = BootHeader::read_from(&mut file)?;
    let keyring = Keyring::derive(password, &Salt::from(boot_header.salt))?;

    let format = resolve_format(boot_header.version)?;

    let context = FormatContext::new(&keyring, config.compression, config.cipher);

    format
        .read_subheader(&mut file, &context)
        .map_err(Error::map_unlock_error)?;

    Ok(VaultSession::new(
        file,
        keyring,
        config.compression,
        config.cipher,
        format,
    ))
}

pub fn create_and_open_vault(
    path: &Path,
    password: &[u8],
    config: CreateConfig,
) -> Result<VaultSession> {
    let resolved_path = resolve_path(path, &config.filename, VAULT_EXTENSION);
    let open_config = OpenConfig::from_create(&config);

    create_vault_with(path, password, config)?;
    open_vault(&resolved_path, password, open_config)
}

pub fn change_password() {
    todo!()
}

pub fn compact() {
    todo!()
}

pub fn close_vault() {
    todo!()
}
