use hkdf::Hkdf;
use sha2::Sha256;

use crate::errors::{Error, Result};

const KDF_INFO: &[u8] = b"openvault-messaging-v1";

pub fn derive_encryption_key(secret: &[u8; 32], salt: &[u8]) -> Result<[u8; 32]> {
    let hkdf = Hkdf::<Sha256>::new(Some(salt), secret);
    let mut okm = [0u8; 32];

    hkdf.expand(KDF_INFO, &mut okm)
        .map_err(|_| Error::HkdfExpandFailed)?;

    Ok(okm)
}
