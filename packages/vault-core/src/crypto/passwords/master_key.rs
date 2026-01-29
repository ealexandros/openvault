use crate::crypto::CryptoError;
use crate::crypto::Result;
use argon2::password_hash::rand_core::RngCore;
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

pub const SALT_LEN: usize = 16;
pub const MASTER_KEY_LEN: usize = 32;

fn generate_salt(len: usize) -> Result<SaltString> {
    let mut bytes = vec![0u8; len];
    OsRng.fill_bytes(&mut bytes);
    SaltString::encode_b64(&bytes).map_err(|e| CryptoError::Kdf(e.to_string()))
}

pub fn generate_new_master_key(password: &[u8]) -> Result<(Zeroizing<String>, SaltString)> {
    let salt = generate_salt(SALT_LEN)?;
    let key = restore_master_key(password, salt.as_str())?;
    Ok((key, salt))
}

pub fn restore_master_key(password: &[u8], salt: &str) -> Result<Zeroizing<String>> {
    let salt = SaltString::from_b64(salt).map_err(|e| CryptoError::Kdf(e.to_string()))?;

    let params = Params::default();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|e| CryptoError::Kdf(e.to_string()))?
        .to_string();

    Ok(Zeroizing::new(password_hash))
}

pub fn derive_subkey(master_key: &[u8], info: &[u8], length: usize) -> Result<Zeroizing<Vec<u8>>> {
    let hk = Hkdf::<Sha256>::new(None, master_key);
    let mut okm = vec![0u8; length];

    hk.expand(info, &mut okm)
        .map_err(|_| CryptoError::Hkdf("HKDF expansion failed".to_string()))?;

    Ok(Zeroizing::new(okm))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_consistency() {
        let password = b"password123";
        let (key1, salt) = generate_new_master_key(password).unwrap();
        let key2 = restore_master_key(password, salt.as_str()).unwrap();

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_kdf_random_salt() {
        let password = b"password123";
        let (key1, salt1) = generate_new_master_key(password).unwrap();
        let (key2, salt2) = generate_new_master_key(password).unwrap();

        assert_ne!(key1, key2);
        assert_ne!(salt1, salt2);
    }
}
