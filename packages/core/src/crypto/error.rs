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

    #[error("Hkdf error: {0}")]
    Hkdf(String),

    #[error("Unknown algorithm: {0}")]
    UnknownAlgorithm(String),
}

pub type Result<T = ()> = std::result::Result<T, CryptoError>;

impl CryptoError {
    pub fn io_enc(e: std::io::Error) -> Self {
        Self::Encryption(format!("I/O Error: {}", e))
    }
    pub fn io_dec(e: std::io::Error) -> Self {
        Self::Decryption(format!("I/O Error: {}", e))
    }
}
