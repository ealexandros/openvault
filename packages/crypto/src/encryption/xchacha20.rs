use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use std::io::{Read, Write};

use crate::encryption::Cipher;
use crate::errors::{Error, Result};

pub const NONCE_LEN: usize = 24;

const DEFAULT_TAG_SIZE: usize = 16;
const DEFAULT_CHUNK_SIZE: usize = 32 * 1024;

#[derive(Debug)]
pub struct XChaCha20Poly1305Cipher;

impl Cipher for XChaCha20Poly1305Cipher {
    fn encrypt(&self, key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|_| Error::InvalidKeyLength)?;

        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|_| Error::EncryptionFailed)?;

        let mut output = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        output.extend_from_slice(nonce.as_slice());
        output.extend_from_slice(&ciphertext);

        Ok(output)
    }

    fn decrypt(&self, key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < NONCE_LEN {
            return Err(Error::DecryptionFailed);
        }

        let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|_| Error::InvalidKeyLength)?;

        let (nonce_bytes, ciphertext) = ciphertext.split_at(NONCE_LEN);
        let nonce = XNonce::from_slice(nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| Error::DecryptionFailed)?;

        Ok(plaintext)
    }

    fn encrypt_stream(&self, key: &[u8], input: &mut dyn Read, output: &mut dyn Write) -> Result {
        let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|_| Error::InvalidKeyLength)?;

        let mut nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        output.write_all(nonce.as_slice()).map_err(Error::Io)?;

        let mut buffer = [0u8; DEFAULT_CHUNK_SIZE];

        loop {
            let n = input.read(&mut buffer).map_err(Error::Io)?;
            if n == 0 {
                break;
            }

            let ciphertext = cipher
                .encrypt(&nonce, &buffer[..n])
                .map_err(|_| Error::EncryptionFailed)?;

            output.write_all(&ciphertext).map_err(Error::Io)?;

            increment_nonce(&mut nonce);
        }

        Ok(())
    }

    fn decrypt_stream(&self, key: &[u8], input: &mut dyn Read, output: &mut dyn Write) -> Result {
        let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|_| Error::InvalidKeyLength)?;

        let mut nonce_bytes = [0u8; NONCE_LEN];
        input.read_exact(&mut nonce_bytes).map_err(Error::Io)?;
        let mut nonce = XNonce::clone_from_slice(&nonce_bytes);

        let mut buffer = [0u8; DEFAULT_CHUNK_SIZE + DEFAULT_TAG_SIZE];

        loop {
            let n = input.read(&mut buffer).map_err(Error::Io)?;
            if n == 0 {
                break;
            }

            let plaintext = cipher
                .decrypt(&nonce, &buffer[..n])
                .map_err(|_| Error::DecryptionFailed)?;

            output.write_all(&plaintext).map_err(Error::Io)?;

            increment_nonce(&mut nonce);
        }

        Ok(())
    }
}

fn increment_nonce(nonce: &mut XNonce) {
    for byte in nonce.iter_mut().rev() {
        *byte = byte.wrapping_add(1);
        if *byte != 0 {
            break;
        }
    }
}
