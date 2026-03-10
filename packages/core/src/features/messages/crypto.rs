use openvault_crypto::keys::{EphemeralPublicKey, SigningPublicKey};
use openvault_crypto::protocols::messaging::{sign_then_encrypt, verify_then_decrypt};

use super::error::Result;
use super::models::MessageCredentials;

pub fn seal_message(
    plaintext: &[u8],
    sender: &MessageCredentials,
    recipient_public_key: &EphemeralPublicKey,
) -> Result<Vec<u8>> {
    sign_then_encrypt(plaintext, &sender.signing_keys, recipient_public_key).map_err(From::from)
}

pub fn open_message(
    ciphertext: &[u8],
    recipient: &MessageCredentials,
    sender_public_key: &SigningPublicKey,
) -> Result<Vec<u8>> {
    let ephemeral_private_key = &recipient.ephemeral_keys.private;
    verify_then_decrypt(ciphertext, ephemeral_private_key, sender_public_key).map_err(From::from)
}
