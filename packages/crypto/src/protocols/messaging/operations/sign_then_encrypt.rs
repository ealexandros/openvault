use crate::errors::Result;
use crate::hash::{Hasher, Sha256Hasher};
use crate::keys::ephemeral::{EphemeralKeyPair, EphemeralPublicKey};
use crate::keys::signing::SigningKeyPair;
use crate::protocols::messaging::kdf::derive_encryption_key;
use crate::protocols::messaging::mapper::encode_payload;
use crate::protocols::messaging::metadata::{
    ENVELOPE_VERSION, EncryptedMessage, EnvelopeConfig, EnvelopeHeader,
};

pub fn sign_then_encrypt(
    message: &[u8],
    sender_signing: &SigningKeyPair,
    recipient_pub: &EphemeralPublicKey,
) -> Result<EncryptedMessage> {
    sign_then_encrypt_with(
        message,
        sender_signing,
        recipient_pub,
        &EnvelopeConfig::default(),
    )
}

pub fn sign_then_encrypt_with(
    message: &[u8],
    sender_signing: &SigningKeyPair,
    recipient_pub: &EphemeralPublicKey,
    config: &EnvelopeConfig,
) -> Result<EncryptedMessage> {
    let message_hash = Sha256Hasher::hash(message);

    let singer = config.signature.resolve();
    let signature = singer.sign(sender_signing.private.as_bytes(), &message_hash);

    let payload = encode_payload(&signature, message)?;
    let compressor = config.compression.resolve();
    let compressed = compressor.compress(&payload)?;

    let ephemeral = EphemeralKeyPair::generate();
    let shared_secret = ephemeral.private.shared_secret(recipient_pub);

    let header = EnvelopeHeader {
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

    Ok(EncryptedMessage { header, ciphertext })
}
