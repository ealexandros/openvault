pub mod factory;
pub mod nonce;
pub mod xchacha20;

pub use factory::EncryptionAlgorithm;
pub use nonce::{NONCE_SIZE, Nonce};

use std::fmt::Debug;

use crate::errors::Result;
use crate::internal::io_ext::{Reader, Writer};

pub trait Cipher: Debug + Send + Sync {
    fn encrypt(&self, key: &[u8], nonce: &Nonce, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, key: &[u8], nonce: &Nonce, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>>;

    fn encrypt_stream(&self, key: &[u8], input: &mut Reader, output: &mut Writer) -> Result;
    fn decrypt_stream(&self, key: &[u8], input: &mut Reader, output: &mut Writer) -> Result;

    fn encrypt_no_aad(&self, key: &[u8], nonce: &Nonce, plaintext: &[u8]) -> Result<Vec<u8>> {
        self.encrypt(key, nonce, plaintext, b"")
    }

    fn decrypt_no_aad(&self, key: &[u8], nonce: &Nonce, ciphertext: &[u8]) -> Result<Vec<u8>> {
        self.decrypt(key, nonce, ciphertext, b"")
    }

    fn encrypt_prefixed_nonce(&self, key: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::random();
        let ciphertext = self.encrypt(key, &nonce, plaintext, aad)?;

        let mut output = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        output.extend_from_slice(nonce.as_bytes());
        output.extend_from_slice(&ciphertext);

        Ok(output)
    }

    fn decrypt_prefixed_nonce(&self, key: &[u8], ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let (nonce, payload) = Nonce::parse_prefixed(ciphertext)?;
        self.decrypt(key, &nonce, payload, aad)
    }
}
