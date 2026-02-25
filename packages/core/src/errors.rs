use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Crypto(#[from] openvault_crypto::errors::Error),

    #[error(transparent)]
    Filesystem(#[from] crate::features::filesystem::FilesystemError),

    // #[error(transparent)]
    // Secrets(#[from] crate::features::secrets::SecretError),
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

    #[error("Feature codec error: {0}")]
    FeatureCodec(String),

    #[error("Unable to unlock vault. Verify password and selected algorithms")]
    UnlockFailed,
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl Error {
    pub fn file_not_exists() -> Self {
        std::io::Error::new(std::io::ErrorKind::NotFound, "File not found").into()
    }
    pub fn map_unlock_error(error: Error) -> Error {
        use openvault_crypto::errors::Error as CryptoError;

        match error {
            Error::Crypto(CryptoError::DecryptionFailed)
            | Error::Crypto(CryptoError::InvalidKeyLength)
            | Error::Crypto(CryptoError::DecompressionFailed(_)) => Error::UnlockFailed,
            _ => error,
        }
    }
}
