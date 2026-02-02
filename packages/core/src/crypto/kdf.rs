use crate::{
    errors::{Error, Result},
    vault::structure,
};
use argon2::password_hash::{
    PasswordHasher, SaltString,
    rand_core::{OsRng, RngCore},
};
use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

pub fn generate_random_salt<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

pub fn generate_default_salt() -> [u8; structure::SALT_LEN] {
    generate_random_salt::<{ structure::SALT_LEN }>()
}

pub fn derive_master_key(password: &[u8], salt: &[u8]) -> Result<Zeroizing<String>> {
    let salt = SaltString::encode_b64(salt).map_err(|e| Error::Kdf(e.to_string()))?;

    let params = Params::default();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|e| Error::Kdf(e.to_string()))?
        .to_string();

    Ok(Zeroizing::new(password_hash))
}

pub fn derive_subkey(master_key: &[u8], info: &[u8]) -> Result<Zeroizing<Vec<u8>>> {
    let hk = Hkdf::<Sha256>::new(None, master_key);
    let mut okm = vec![0u8; structure::SUB_KEY_LEN];

    hk.expand(info, &mut okm)
        .map_err(|_| Error::Hkdf("HKDF expansion failed".to_string()))?;

    Ok(Zeroizing::new(okm))
}
