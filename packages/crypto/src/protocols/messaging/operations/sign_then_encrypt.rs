use crate::errors::Result;
use crate::hash::{Hasher, Sha256Hasher};
use crate::keys::ephemeral::{EphemeralKeyPair, EphemeralPublicKey};
use crate::keys::signing::SigningKeyPair;
use crate::protocols::messaging::kdf::derive_encryption_key;
use crate::protocols::messaging::mapper::{encode_message, encode_payload};
use crate::protocols::messaging::metadata::{
    ENVELOPE_VERSION, MessageConfig, MessageEnvelope, MessageHeader,
};

pub fn sign_then_encrypt(
    payload: &[u8],
    sender_signing: &SigningKeyPair,
    recipient_pub: &EphemeralPublicKey,
) -> Result<Vec<u8>> {
    sign_then_encrypt_with(
        payload,
        sender_signing,
        recipient_pub,
        &MessageConfig::default(),
    )
}

pub fn sign_then_encrypt_with(
    payload: &[u8],
    sender_signing: &SigningKeyPair,
    recipient_pub: &EphemeralPublicKey,
    config: &MessageConfig,
) -> Result<Vec<u8>> {
    let payload_hash = Sha256Hasher::hash(payload);

    let singer = config.signature.resolve();
    let signature = singer.sign(sender_signing.private.as_bytes(), &payload_hash);

    let payload = encode_payload(&signature, payload)?;
    let compressor = config.compression.resolve();
    let compressed = compressor.compress(&payload)?;

    let ephemeral = EphemeralKeyPair::generate();
    let shared_secret = ephemeral.private.shared_secret(recipient_pub);

    let header = MessageHeader {
        version: ENVELOPE_VERSION,
        hash: config.hash,
        signature: config.signature,
        kdf: config.kdf,
        encryption: config.encryption,
        compression: config.compression,
        ephemeral_public_key: ephemeral.public.to_bytes(),
    };

    let aad = header.aad_bytes();
    let key = derive_encryption_key(&shared_secret, &aad)?;
    let cipher = config.encryption.resolve();
    let ciphertext = cipher.encrypt_prefixed_nonce(&key, &compressed, &aad)?;

    let message_envelope = MessageEnvelope { header, ciphertext };

    encode_message(&message_envelope)
}
