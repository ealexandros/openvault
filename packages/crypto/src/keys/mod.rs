use argon2::password_hash::rand_core::{OsRng, RngCore};
use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::errors::{Error, Result};

pub const MKEY_SIZE: usize = 32;
pub const SALT_SIZE: usize = 16;

pub type Salt = [u8; SALT_SIZE];

#[derive(Debug, Clone, PartialEq)]
pub struct MasterKey(Zeroizing<[u8; MKEY_SIZE]>);

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedKey<const N: usize = MKEY_SIZE>(Zeroizing<[u8; N]>);

impl MasterKey {
    pub fn new(key: [u8; MKEY_SIZE]) -> Self {
        Self(Zeroizing::new(key))
    }

    pub fn as_bytes(&self) -> &[u8; MKEY_SIZE] {
        &self.0
    }

    pub fn derive(password: &[u8], salt: &[u8]) -> Result<Self> {
        let params = Params::default();
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut key = [0u8; MKEY_SIZE];

        argon2
            .hash_password_into(password, salt, &mut key)
            .map_err(|_| Error::KeyDerivationFailed)?;

        Ok(Self(Zeroizing::new(key)))
    }

    pub fn derive_with_random_salt(password: &[u8]) -> Result<(Self, Salt)> {
        let salt = random_salt();
        let key = Self::derive(password, &salt)?;
        Ok((key, salt))
    }

    pub fn expand<const N: usize>(&self, info: &[u8]) -> Result<DerivedKey<N>> {
        let hkdf = Hkdf::<Sha256>::new(None, self.as_bytes());
        let mut okm = [0u8; N];

        hkdf.expand(info, &mut okm)
            .map_err(|_| Error::HkdfExpandFailed)?;

        Ok(DerivedKey(Zeroizing::new(okm)))
    }
}

pub fn random_salt() -> Salt {
    let mut salt = Salt::default();
    OsRng.fill_bytes(&mut salt);
    salt
}
