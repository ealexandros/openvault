use crate::{
    constants::{KEY_LEN, SALT_LEN},
    errors::{KdfError, Result},
};
use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

pub fn generate_default_salt() -> [u8; SALT_LEN] {
    super::rand::random_bytes::<SALT_LEN>()
}

pub fn derive_master_key(password: &[u8], salt: &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>> {
    let params = Params::default();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; KEY_LEN];

    argon2
        .hash_password_into(password, salt, &mut key)
        .map_err(|_| KdfError::DerivationFailed)?;

    Ok(Zeroizing::new(key))
}

pub fn derive_subkey(master_key: &[u8], info: &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>> {
    let hkdf = Hkdf::<Sha256>::new(None, master_key);
    let mut okm = [0u8; KEY_LEN];

    hkdf.expand(info, &mut okm)
        .map_err(|_| KdfError::HkdfExpand)?;

    Ok(Zeroizing::new(okm))
}
