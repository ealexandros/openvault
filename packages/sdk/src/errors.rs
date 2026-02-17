use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Crypto(#[from] openvault_crypto::errors::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),

    #[error("Invalid vault checksum")]
    InvalidVaultChecksum,

    #[error("Invalid vault format")]
    InvalidVaultFormat,

    #[error("Unsupported vault version: {0}")]
    UnsupportedVaultVersion(u16),

    #[error("Invalid path")]
    InvalidPath,

    #[error("Invalid entry header")]
    InvalidEntryHeader,
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl Error {
    pub fn file_exists() -> Self {
        std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists").into()
    }
}
