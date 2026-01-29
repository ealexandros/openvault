use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
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

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Hkdf error: {0}")]
    Hkdf(String),

    #[error("Unknown algorithm: {0}")]
    UnknownAlgorithm(String),

    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, CryptoError>;
