use openvault_crypto::keys::{EphemeralPublicKey, SigningPublicKey};
use openvault_crypto::protocols::messaging::{
    EncryptedMessage, sign_then_encrypt, verify_then_decrypt,
};

use super::error::Result;
use super::models::MessageCredentials;

pub type MessageEnvelope = EncryptedMessage;

pub fn seal_message(
    plaintext: &[u8],
    sender: &MessageCredentials,
    recipient_public_key: &EphemeralPublicKey,
) -> Result<MessageEnvelope> {
    sign_then_encrypt(plaintext, &sender.signing_keys, recipient_public_key).map_err(From::from)
}

pub fn open_message(
    envelope: &MessageEnvelope,
    recipient: &MessageCredentials,
    sender_public_key: &SigningPublicKey,
) -> Result<Vec<u8>> {
    verify_then_decrypt(
        envelope,
        &recipient.ephemeral_keys.private,
        sender_public_key,
    )
    .map_err(From::from)
}
