use super::{Compressor, zstd};
use crate::crypto::{CryptoError, Result};

pub fn get(algorithm: &str) -> Result<Box<dyn Compressor>> {
    match algorithm.to_ascii_lowercase().as_str() {
        "zstd" => Ok(Box::new(zstd::Zstd)),
        _ => Err(CryptoError::UnknownAlgorithm(algorithm.to_string())),
    }
}
