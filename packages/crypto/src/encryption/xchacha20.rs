use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};

use crate::encryption::{Cipher, NONCE_SIZE, Nonce};
use crate::errors::{Error, Result};
use crate::internal::io_ext::{Reader, Writer};

const DEFAULT_TAG_SIZE: usize = 16;
const DEFAULT_CHUNK_SIZE: usize = 32 * 1024;

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

    fn encrypt_stream(&self, key: &[u8], input: &mut Reader, output: &mut Writer) -> Result {
        let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|_| Error::InvalidKeyLength)?;

        let mut nonce = Nonce::random();
        output.write_all(nonce.as_bytes()).map_err(Error::Io)?;

        let mut buffer = [0u8; DEFAULT_CHUNK_SIZE];

        loop {
            let n = input.read(&mut buffer).map_err(Error::Io)?;
            if n == 0 {
                break;
            }

            let offset = output.stream_position()?;

            let x_nonce = XNonce::from_slice(nonce.as_bytes());
            let payload = Payload {
                msg: &buffer[..n],
                aad: &offset.to_le_bytes(),
            };

            let ciphertext = cipher
                .encrypt(x_nonce, payload)
                .map_err(|_| Error::EncryptionFailed)?;

            output.write_all(&ciphertext).map_err(Error::Io)?;

            nonce.increment();
        }

        Ok(())
    }

    fn decrypt_stream(&self, key: &[u8], input: &mut Reader, output: &mut Writer) -> Result {
        let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|_| Error::InvalidKeyLength)?;

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        input.read_exact(&mut nonce_bytes).map_err(Error::Io)?;

        let mut nonce = Nonce::new(nonce_bytes);

        let mut buffer = [0u8; DEFAULT_CHUNK_SIZE + DEFAULT_TAG_SIZE];

        loop {
            let offset = input.stream_position()?;

            let n = input.read(&mut buffer).map_err(Error::Io)?;
            if n == 0 {
                break;
            }

            let x_nonce = XNonce::from_slice(nonce.as_bytes());
            let payload = Payload {
                msg: &buffer[..n],
                aad: &offset.to_le_bytes(),
            };

            let plaintext = cipher
                .decrypt(x_nonce, payload)
                .map_err(|_| Error::DecryptionFailed)?;

            output.write_all(&plaintext).map_err(Error::Io)?;

            nonce.increment();
        }

        Ok(())
    }
}
