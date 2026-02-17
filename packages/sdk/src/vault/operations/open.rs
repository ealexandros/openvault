use std::fs::File;
use std::path::Path;

use openvault_crypto::keys::MasterKey;

use crate::domain::Vault;
use crate::errors::Result;
use crate::versions;
use crate::versions::header::VaultHeader;

pub fn open_vault(path: String, password: String) -> Result<Vault> {
    let path = Path::new(&path);

    let mut file = File::open(path)?;

    let header = VaultHeader::read_from(&mut file)?;
    let handler = versions::get_handler(header.version)?;

    let key = MasterKey::derive(password.as_bytes(), &header.salt)?;

    let vault = handler.read(&mut file, &key)?;

    Ok(vault)
}
