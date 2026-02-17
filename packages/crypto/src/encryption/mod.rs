use std::fmt::Debug;
use std::io::{Read, Write};

use crate::errors::Result;

pub const NONCE_SIZE: usize = 24;

pub use factory::EncryptionAlgorithm;

pub trait Cipher: Debug + Send + Sync {
    fn encrypt(&self, key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>>;
    fn encrypt_stream(&self, key: &[u8], input: &mut dyn Read, output: &mut dyn Write) -> Result;
    fn decrypt_stream(&self, key: &[u8], input: &mut dyn Read, output: &mut dyn Write) -> Result;
}

pub mod factory;
pub mod xchacha20;
