use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::errors::{Error, Result};
use crate::keys::derived_key::DerivedKey;
use crate::keys::salt::Salt;
use crate::memory::SecretSlice;

pub const MKEY_SIZE: usize = 32;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct MasterKey {
    key: SecretSlice<MKEY_SIZE>,
}

impl MasterKey {
    pub fn new(bytes: [u8; MKEY_SIZE]) -> Result<Self> {
        let key = SecretSlice::new(bytes).map_err(|_| Error::MemoryLockFailed)?;
        Ok(Self { key })
    }

    pub fn derive(password: &[u8], salt: &Salt) -> Result<Self> {
        let params = Params::default();
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        let mut raw = [0u8; MKEY_SIZE];
        argon2
            .hash_password_into(password, salt.as_bytes(), &mut raw)
            .map_err(|_| Error::KeyDerivationFailed)?;

        let master = Self::new(raw)?;
        raw.zeroize();

        Ok(master)
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

        DerivedKey::new(okm)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.key.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_key_new() {
        let key = MasterKey::new([0u8; MKEY_SIZE]).unwrap();
        assert_eq!(key.as_bytes(), &[0u8; MKEY_SIZE]);
    }

    #[test]
    fn test_master_key_as_ref() {
        let key = MasterKey::new([0u8; MKEY_SIZE]).unwrap();
        assert_eq!(key.as_bytes(), &[0u8; MKEY_SIZE]);
    }
}
