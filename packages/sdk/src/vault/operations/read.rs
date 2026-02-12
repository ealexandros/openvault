use std::path::Path;
use thiserror::Error;

use crate::domain::Vault;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Crypto error")]
    Crypto,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid vault format")]
    InvalidFormat,
}

pub type Result<T> = std::result::Result<T, OperationError>;

pub fn open_vault<P: AsRef<Path>>(_path: P, _password: &str) -> Result<Vault> {
    // @todo-now: Implement vault opening logic: read header, derive key, decrypt metadata, parse V1/V2, map to domain
    todo!()
}
