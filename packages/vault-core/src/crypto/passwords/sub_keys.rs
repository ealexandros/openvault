use crate::crypto::{CryptoError, Result};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

pub fn derive_subkey(master_key: &[u8], info: &[u8], length: usize) -> Result<Zeroizing<Vec<u8>>> {
    let hk = Hkdf::<Sha256>::new(None, master_key);
    let mut okm = vec![0u8; length];

    hk.expand(info, &mut okm)
        .map_err(|_| CryptoError::Hkdf("HKDF expansion failed".to_string()))?;

    Ok(Zeroizing::new(okm))
}
