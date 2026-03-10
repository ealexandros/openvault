use crate::errors::{Error, Result};
use crate::hash::{Hasher, Sha256Hasher};
use crate::keys::ephemeral::{EphemeralPrivateKey, EphemeralPublicKey};
use crate::keys::signing::SigningPublicKey;
use crate::protocols::messaging::kdf::derive_encryption_key;
use crate::protocols::messaging::mapper::decode_payload;
use crate::protocols::messaging::metadata::{EncryptedMessage, HashAlgorithm, KdfAlgorithm};

pub fn verify_then_decrypt(
    envelope: &EncryptedMessage,
    recipient_private: &EphemeralPrivateKey,
    sender_public: &SigningPublicKey,
) -> Result<Vec<u8>> {
    envelope.header.ensure_supported()?;

    if envelope.header.hash != HashAlgorithm::Sha256 {
        return Err(Error::InvalidEnvelope);
    }

    if envelope.header.kdf != KdfAlgorithm::HkdfSha256 {
        return Err(Error::InvalidEnvelope);
    }

    let peer_public = EphemeralPublicKey::from_bytes(envelope.header.ephemeral_public_key);
    let shared_secret = recipient_private.shared_secret(&peer_public);

    let aad = envelope.header.aad_bytes();
    let key = derive_encryption_key(&shared_secret, &aad)?;

    let cipher = envelope.header.encryption.resolve();
    let compressed = cipher.decrypt_prefixed_nonce(&key, &envelope.ciphertext, &aad)?;
    let compressor = envelope.header.compression.resolve();
    let payload = compressor.decompress(&compressed)?;

    let (signature, message) = decode_payload(&payload)?;
    if signature.len() != 64 {
        return Err(Error::InvalidEnvelope);
    }

    let message_hash = Sha256Hasher::hash(&message);
    let signer = envelope.header.signature.resolve();

    if !signer.verify(sender_public.as_bytes(), &message_hash, &signature) {
        return Err(Error::SignatureVerificationFailed);
    }

    Ok(message)
}
