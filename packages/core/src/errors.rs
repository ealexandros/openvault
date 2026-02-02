use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Decompression error: {0}")]
    Decompression(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Key derivation error: {0}")]
    Kdf(String),

    #[error("Invalid key length")]
    InvalidKeyLength,

    #[error("Hkdf error: {0}")]
    Hkdf(String),

    #[error("Unknown algorithm: {0}")]
    UnknownAlgorithm(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Bincode error: {0}")]
    Bincode(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("WalkDir error: {0}")]
    WalkDir(String),

    #[error("Vault error: {0}")]
    Vault(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl Error {
    pub fn io_enc(e: io::Error) -> Self {
        Self::Encryption(format!("I/O Error: {}", e))
    }
    pub fn io_dec(e: io::Error) -> Self {
        Self::Decryption(format!("I/O Error: {}", e))
    }
    pub fn dir_not_found() -> Self {
        let error = io::Error::new(io::ErrorKind::NotFound, "Directory not found");
        Self::Io(error)
    }
    pub fn dir_not_a_dir() -> Self {
        let error = io::Error::new(io::ErrorKind::InvalidInput, "Not a directory");
        Self::Io(error)
    }
    pub fn file_exists() -> Self {
        let error = io::Error::new(io::ErrorKind::AlreadyExists, "File already exists");
        Self::Io(error)
    }
    pub fn file_not_found() -> Self {
        let error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        Self::Io(error)
    }
    pub fn file_not_a_file() -> Self {
        let error = io::Error::new(io::ErrorKind::InvalidInput, "Not a file");
        Self::Io(error)
    }
}
