use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};

use crate::encryption::{Cipher, Nonce};
use crate::errors::{Error, Result};

#[derive(Debug)]
pub struct XChaCha20Poly1305Cipher;

impl Cipher for XChaCha20Poly1305Cipher {
    fn encrypt(&self, key: &[u8], nonce: &Nonce, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|_| Error::InvalidKeyLength)?;
        let nonce = XNonce::from_slice(nonce.as_bytes());

        let payload = Payload {
            msg: plaintext,
            aad,
        };
        let ciphertext = cipher
            .encrypt(nonce, payload)
            .map_err(|_| Error::EncryptionFailed)?;

        Ok(ciphertext)
    }

    fn decrypt(&self, key: &[u8], nonce: &Nonce, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|_| Error::InvalidKeyLength)?;
        let nonce = XNonce::from_slice(nonce.as_bytes());

        let payload = Payload {
            msg: ciphertext,
            aad,
        };
        let plaintext = cipher
            .decrypt(nonce, payload)
            .map_err(|_| Error::DecryptionFailed)?;

        Ok(plaintext)
    }
}
