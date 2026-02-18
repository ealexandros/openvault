use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, PartialEq)]
pub struct EncryptedField(Vec<u8>);

impl EncryptedField {
    pub fn new(ciphertext: Vec<u8>) -> Self {
        Self(ciphertext)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
