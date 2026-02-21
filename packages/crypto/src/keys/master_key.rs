use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::errors::{Error, Result};
use crate::keys::derived_key::DerivedKey;
use crate::keys::salt::Salt;

pub const MKEY_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq)]
pub struct MasterKey(Zeroizing<[u8; MKEY_SIZE]>);

impl MasterKey {
    pub fn new(key: [u8; MKEY_SIZE]) -> Self {
        Self(Zeroizing::new(key))
    }

    pub fn derive(password: &[u8], salt: &Salt) -> Result<Self> {
        let params = Params::default();

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        let mut key = [0u8; MKEY_SIZE];
        argon2
            .hash_password_into(password, salt.as_bytes(), &mut key)
            .map_err(|_| Error::KeyDerivationFailed)?;

        Ok(Self(Zeroizing::new(key)))
    }

    pub fn derive_with_random_salt(password: &[u8]) -> Result<(Self, Salt)> {
        let salt = Salt::random();
        let key = Self::derive(password, &salt)?;
        Ok((key, salt))
    }

    pub fn expand<const N: usize>(&self, info: &[u8]) -> Result<DerivedKey<N>> {
        let hkdf = Hkdf::<Sha256>::new(None, self.as_bytes());
        let mut okm = [0u8; N];

        hkdf.expand(info, &mut okm)
            .map_err(|_| Error::HkdfExpandFailed)?;

        Ok(DerivedKey::new(okm))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_key_new() {
        let key = MasterKey::new([0u8; MKEY_SIZE]);
        assert_eq!(key.as_bytes(), [0u8; MKEY_SIZE]);
    }

    #[test]
    fn test_master_key_as_ref() {
        let key = MasterKey::new([0u8; MKEY_SIZE]);
        assert_eq!(key.as_bytes(), [0u8; MKEY_SIZE]);
    }
}
