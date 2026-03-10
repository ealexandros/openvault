use serde::{Deserialize, Serialize};

use super::error::{MessagesError, Result};
use super::models::{MessageCredentials, PrivateKey};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageEnvelope {
    pub ciphertext: Vec<u8>,
}

impl MessageEnvelope {
    pub fn new(ciphertext: Vec<u8>) -> Result<Self> {
        if ciphertext.is_empty() {
            return Err(MessagesError::InvalidInput(
                "Ciphertext must not be empty".to_string(),
            ));
        }

        Ok(Self { ciphertext })
    }
}

pub fn seal_message(
    _plaintext: &[u8],
    _sender: &MessageCredentials,
    _recipient_public_key: &[u8],
) -> Result<MessageEnvelope> {
    Err(MessagesError::CryptoUnavailable)
}

pub fn open_message(
    _envelope: &MessageEnvelope,
    _recipient_private_key: &PrivateKey,
    _sender_public_key: &[u8],
) -> Result<Vec<u8>> {
    Err(MessagesError::CryptoUnavailable)
}
