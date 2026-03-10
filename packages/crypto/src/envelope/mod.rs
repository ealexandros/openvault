pub mod sign_then_encrypt;
pub mod verify_then_decrypt;

pub use sign_then_encrypt::{sign_then_encrypt, sign_then_encrypt_with};
pub use verify_then_decrypt::verify_then_decrypt;

use hkdf::Hkdf;
use sha2::Sha256;

use crate::encryption::EncryptionAlgorithm;
use crate::errors::{Error, Result};
use crate::protocol::{
    CompressionMode, EnvelopeHeader, HashAlgorithm, KdfAlgorithm, SignatureAlgorithm,
};

pub struct EnvelopeConfig {
    pub compression: CompressionMode,
    pub hash: HashAlgorithm,
    pub signature: SignatureAlgorithm,
    pub kdf: KdfAlgorithm,
    pub encryption: EncryptionAlgorithm,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        Self {
            compression: CompressionMode::Zstd,
            hash: HashAlgorithm::Sha256,
            signature: SignatureAlgorithm::Ed25519,
            kdf: KdfAlgorithm::HkdfSha256,
            encryption: EncryptionAlgorithm::XChaCha20Poly1305,
        }
    }
}

const SIGNATURE_LEN_BYTES: usize = 2;
const KDF_INFO: &[u8] = b"openvault-envelope-v1";

pub(crate) fn derive_encryption_key(
    shared_secret: &[u8; 32],
    header: &EnvelopeHeader,
) -> Result<[u8; 32]> {
    let salt = header.aad_bytes();
    let hkdf = Hkdf::<Sha256>::new(Some(&salt), shared_secret);
    let mut okm = [0u8; 32];

    hkdf.expand(KDF_INFO, &mut okm)
        .map_err(|_| Error::HkdfExpandFailed)?;

    Ok(okm)
}

pub(crate) fn encode_payload(signature: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let sig_len = signature.len();

    if sig_len > u16::MAX as usize {
        return Err(Error::InvalidEnvelope);
    }

    let mut output = Vec::with_capacity(SIGNATURE_LEN_BYTES + sig_len + message.len());

    output.extend_from_slice(&(sig_len as u16).to_be_bytes());
    output.extend_from_slice(signature);
    output.extend_from_slice(message);

    Ok(output)
}

pub(crate) fn decode_payload(payload: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    if payload.len() < SIGNATURE_LEN_BYTES {
        return Err(Error::InvalidEnvelope);
    }

    let sig_len = u16::from_be_bytes([payload[0], payload[1]]) as usize;
    let signature_start = SIGNATURE_LEN_BYTES;
    let signature_end = signature_start + sig_len;

    if payload.len() < signature_end {
        return Err(Error::InvalidEnvelope);
    }

    let signature = payload[signature_start..signature_end].to_vec();
    let message = payload[signature_end..].to_vec();

    Ok((signature, message))
}
