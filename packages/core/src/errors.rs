use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),

    #[error("KDF derivation failed")]
    KdfDerivationFailed,

    #[error("HKDF expand failed")]
    HkdfExpandFailed,

    #[error("Encryption failed")]
    EncryptionFailed,

    #[error("Decryption failed")]
    DecryptionFailed,

    #[error("Invalid key length")]
    InvalidKeyLength,

    #[error("Compression failed: {0}")]
    CompressionFailed(String),

    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    #[error("Invalid vault magic")]
    InvalidVaultMagic,

    #[error("Invalid vault checksum")]
    InvalidVaultChecksum,

    #[error("Invalid vault format")]
    InvalidVaultFormat,

    #[error("Unsupported vault version")]
    UnsupportedVaultVersion,

    #[error("Invalid path")]
    InvalidPath,

    #[error("Unsupported command: {0}")]
    UnsupportedCommand(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl Error {
    pub fn dir_not_found() -> Self {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Directory not found").into()
    }
    pub fn dir_not_a_dir() -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a directory").into()
    }
    pub fn file_exists() -> Self {
        std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists").into()
    }
    pub fn file_not_found() -> Self {
        std::io::Error::new(std::io::ErrorKind::NotFound, "File not found").into()
    }
    pub fn file_not_a_file() -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a file").into()
    }
}
