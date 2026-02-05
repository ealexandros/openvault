use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdfError {
    #[error("HKDF expand failed")]
    HkdfExpand,

    #[error("Invalid key length")]
    InvalidKeyLength,

    #[error("Key derivation failed")]
    DerivationFailed,
}

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Encryption failed")]
    Encryption,

    #[error("Decryption failed")]
    Decryption,

    #[error("Invalid key length")]
    InvalidKeyLength,
}

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("Invalid vault magic")]
    InvalidMagic,

    #[error("Invalid vault checksum")]
    InvalidChecksum,

    #[error("Invalid vault format")]
    InvalidFormat,

    #[error("Unsupported vault version")]
    UnsupportedVersion,

    #[error("WalkDir error: {0}")]
    WalkDir(String),

    #[error("Vault I/O error")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Generic(String),
}

#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Compression failed: {0}")]
    Compress(String),

    #[error("Decompression failed: {0}")]
    Decompress(String),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Compression(#[from] CompressionError),

    #[error(transparent)]
    Kdf(#[from] KdfError),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Vault(#[from] VaultError),

    #[error("WalkDir error: {0}")]
    WalkDir(String),

    #[error("Invalid path")]
    InvalidPath,

    #[error("Unsupported command: {0}")]
    Unsupported(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl Error {
    pub fn dir_not_found() -> Self {
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, "Directory not found");
        Self::Io(error)
    }
    pub fn dir_not_a_dir() -> Self {
        let error = std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a directory");
        Self::Io(error)
    }
    pub fn file_exists() -> Self {
        let error = std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists");
        Self::Io(error)
    }
    pub fn file_not_found() -> Self {
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        Self::Io(error)
    }
    pub fn file_not_a_file() -> Self {
        let error = std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a file");
        Self::Io(error)
    }
}
