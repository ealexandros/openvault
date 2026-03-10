use crate::envelope::{decode_payload, derive_encryption_key};
use crate::errors::{Error, Result};
use crate::hash::{Hasher, Sha256Hasher};
use crate::keys::ephemeral::{EphemeralPrivateKey, EphemeralPublicKey};
use crate::keys::signing::SigningPublicKey;
use crate::protocol::{EncryptedMessage, HashAlgorithm, KdfAlgorithm, SignatureAlgorithm};
use crate::signature::{Ed25519Signer, Signer};

pub fn verify_then_decrypt(
    envelope: &EncryptedMessage,
    recipient_private: &EphemeralPrivateKey,
    sender_public: &SigningPublicKey,
) -> Result<Vec<u8>> {
    envelope.header.ensure_supported()?;

    if envelope.header.hash != HashAlgorithm::Sha256 {
        return Err(Error::InvalidEnvelope);
    }

    if envelope.header.signature != SignatureAlgorithm::Ed25519 {
        return Err(Error::InvalidEnvelope);
    }

    if envelope.header.kdf != KdfAlgorithm::HkdfSha256 {
        return Err(Error::InvalidEnvelope);
    }

    let peer_public = EphemeralPublicKey::from_bytes(envelope.header.ephemeral_public_key);
    let shared_secret = recipient_private.shared_secret(&peer_public);
    let key = derive_encryption_key(&shared_secret, &envelope.header)?;

    let aad = envelope.header.aad_bytes();
    let cipher = envelope.header.encryption.resolve();
    let compressed = cipher.decrypt_prefixed_nonce(&key, &envelope.ciphertext, &aad)?;
    let payload = envelope.header.compression.decompress(&compressed)?;

    let (signature, message) = decode_payload(&payload)?;
    if signature.len() != 64 {
        return Err(Error::InvalidEnvelope);
    }

    let message_hash = Sha256Hasher::hash(&message);
    if !Ed25519Signer::verify(sender_public, &message_hash, &signature) {
        return Err(Error::SignatureVerificationFailed);
    }

    Ok(message)
}
