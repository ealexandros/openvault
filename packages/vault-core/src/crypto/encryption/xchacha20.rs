use super::Cipher;
use crate::crypto::error::{CryptoError, Result};
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use std::io::{Read, Write};

#[derive(Debug, Default, Clone, Copy)]
pub struct XChaCha20Poly1305Cipher;

const KEY_SIZE: usize = 32;
const NONCE_SIZE: usize = 24;
const CHUNK_SIZE: usize = 32 * 1024;

// @todo-now change the nonce every time..

impl Cipher for XChaCha20Poly1305Cipher {
    fn encrypt(&self, key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        if key.len() != KEY_SIZE {
            return Err(CryptoError::InvalidKeyLength);
        }

        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|_| CryptoError::Encryption("Encryption failed".to_string()))?;

        Ok((ciphertext, nonce.to_vec()))
    }

    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        if key.len() != KEY_SIZE {
            return Err(CryptoError::InvalidKeyLength);
        }
        if nonce.len() != NONCE_SIZE {
            return Err(CryptoError::Decryption("Invalid nonce length".to_string()));
        }

        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce = XNonce::from_slice(nonce);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::Decryption("Decryption failed".to_string()))?;

        Ok(plaintext)
    }

    fn encrypt_stream(
        &self,
        key: &[u8],
        input: &mut dyn Read,
        output: &mut dyn Write,
    ) -> Result<()> {
        if key.len() != KEY_SIZE {
            return Err(CryptoError::InvalidKeyLength);
        }

        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let nonce_bytes = nonce.as_slice();

        output
            .write_all(nonce_bytes)
            .map_err(|e| CryptoError::Encryption(format!("Failed to write nonce: {}", e)))?;

        let mut buffer = [0u8; CHUNK_SIZE];

        loop {
            let n = input
                .read(&mut buffer)
                .map_err(|e| CryptoError::Encryption(format!("Read failed: {}", e)))?;

            if n == 0 {
                break;
            }

            let chunk = &buffer[..n];
            let ciphertext = cipher
                .encrypt(&nonce, chunk)
                .map_err(|_| CryptoError::Encryption("Chunk encryption failed".to_string()))?;
            output
                .write_all(&ciphertext)
                .map_err(|e| CryptoError::Encryption(format!("Write failed: {}", e)))?;
        }

        Ok(())
    }

    fn decrypt_stream(
        &self,
        key: &[u8],
        input: &mut dyn Read,
        output: &mut dyn Write,
    ) -> Result<()> {
        if key.len() != KEY_SIZE {
            return Err(CryptoError::InvalidKeyLength);
        }

        let cipher = XChaCha20Poly1305::new(key.into());

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        input
            .read_exact(&mut nonce_bytes)
            .map_err(|e| CryptoError::Decryption(format!("Failed to read nonce: {}", e)))?;
        let nonce = XNonce::from_slice(&nonce_bytes);

        let mut buffer = [0u8; CHUNK_SIZE + 16];

        loop {
            let n = input
                .read(&mut buffer)
                .map_err(|e| CryptoError::Decryption(format!("Read failed: {}", e)))?;
            if n == 0 {
                break;
            }

            let chunk = &buffer[..n];
            let plaintext = cipher
                .decrypt(nonce, chunk)
                .map_err(|_| CryptoError::Decryption("Chunk decryption failed".to_string()))?;
            output
                .write_all(&plaintext)
                .map_err(|e| CryptoError::Decryption(format!("Write failed: {}", e)))?;
        }

        Ok(())
    }
}
