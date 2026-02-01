use crate::crypto::{CryptoError, Result};
use argon2::password_hash::{
    PasswordHasher, SaltString,
    rand_core::{OsRng, RngCore},
};
use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

pub const SALT_LEN: usize = 16;
pub const MASTER_KEY_LEN: usize = 32;
pub const SUB_KEY_LEN: usize = 32;

pub fn generate_random_salt(len: usize) -> Result<SaltString> {
    let mut bytes = vec![0u8; len];
    OsRng.fill_bytes(&mut bytes);
    SaltString::encode_b64(&bytes).map_err(|e| CryptoError::Kdf(e.to_string()))
}

pub fn derive_master_key(password: &[u8], salt: &str) -> Result<Zeroizing<String>> {
    let salt = SaltString::from_b64(salt).map_err(|e| CryptoError::Kdf(e.to_string()))?;

    let params = Params::default();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|e| CryptoError::Kdf(e.to_string()))?
        .to_string();

    Ok(Zeroizing::new(password_hash))
}

pub fn derive_subkey(master_key: &[u8], info: &[u8]) -> Result<Zeroizing<Vec<u8>>> {
    let hk = Hkdf::<Sha256>::new(None, master_key);
    let mut okm = vec![0u8; SUB_KEY_LEN];

    hk.expand(info, &mut okm)
        .map_err(|_| CryptoError::Hkdf("HKDF expansion failed".to_string()))?;

    Ok(Zeroizing::new(okm))
}
