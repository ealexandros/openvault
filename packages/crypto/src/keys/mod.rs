use argon2::password_hash::rand_core::{OsRng, RngCore};
use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::errors::{Error, Result};

pub const MKEY_LEN: usize = 32;
pub const SALT_LEN: usize = 16;

#[derive(Debug, Clone, PartialEq)]
pub struct MasterKey(Zeroizing<[u8; MKEY_LEN]>);

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedKey<const N: usize = MKEY_LEN>(Zeroizing<[u8; N]>);

impl MasterKey {
    pub fn new(key: [u8; MKEY_LEN]) -> Self {
        Self(Zeroizing::new(key))
    }

    pub fn as_bytes(&self) -> &[u8; MKEY_LEN] {
        &self.0
    }

    pub fn derive(password: &[u8], salt: &[u8]) -> Result<Self> {
        let params = Params::default();
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut key = [0u8; MKEY_LEN];

        argon2
            .hash_password_into(password, salt, &mut key)
            .map_err(|_| Error::KeyDerivationFailed)?;

        Ok(Self(Zeroizing::new(key)))
    }

    pub fn expand<const N: usize>(&self, info: &[u8]) -> Result<DerivedKey<N>> {
        let hkdf = Hkdf::<Sha256>::new(None, self.as_bytes());
        let mut okm = [0u8; N];

        hkdf.expand(info, &mut okm)
            .map_err(|_| Error::HkdfExpandFailed)?;

        Ok(DerivedKey(Zeroizing::new(okm)))
    }
}

pub fn generate_default_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    salt
}
