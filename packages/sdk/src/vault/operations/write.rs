use std::path::Path;

use crate::domain::Vault;
use crate::errors::Result;

pub fn create_vault<P: AsRef<Path>>(_path: P, _password: &str) -> Result<Vault> {
    // @todo-now: Implement vault creation logic: generate salt/keys, init empty V1 structure, encrypt and write header+metadata
    todo!()
}

pub fn save_vault<P: AsRef<Path>>(_vault: &Vault, _path: P, _password: &str) -> Result<()> {
    // @todo-now: Implement save logic: map domain to schema, encrypt, write to file
    todo!()
}
