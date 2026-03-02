use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sdk(#[from] openvault_sdk::Error),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),

    #[error("Vault not opened")]
    VaultNotOpened,

    #[error("Invalid encryption algorithm: {0}")]
    InvalidEncryption(String),

    #[error("Invalid compression algorithm: {0}")]
    InvalidCompression(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;
