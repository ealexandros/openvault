use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Key derivation failed")]
    KeyDerivationFailed,

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

    #[error("Unsupported cipher: {0}")]
    UnsupportedCipher(u8),

    #[error("Unsupported compressor: {0}")]
    UnsupportedCompressor(u8),
}

pub type Result<T = ()> = std::result::Result<T, Error>;
