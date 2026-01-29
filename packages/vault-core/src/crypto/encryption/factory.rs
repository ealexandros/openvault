use super::{Cipher, xchacha20};
use crate::crypto::{CryptoError, Result};

pub fn get(algorithm: &str) -> Result<Box<dyn Cipher>> {
    match algorithm.to_ascii_lowercase().as_str() {
        "xchacha20poly1305" | "xchacha20" => Ok(Box::new(xchacha20::XChaCha20Poly1305Cipher)),
        _ => Err(CryptoError::UnknownAlgorithm(algorithm.to_string())),
    }
}
