use argon2::password_hash::rand_core::{OsRng, RngCore};

pub const SALT_SIZE: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Salt([u8; SALT_SIZE]);

impl Salt {
    pub fn new(bytes: [u8; SALT_SIZE]) -> Self {
        Self(bytes)
    }

    pub fn random() -> Self {
        let mut bytes = [0u8; SALT_SIZE];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; SALT_SIZE] {
        &self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> [u8; SALT_SIZE] {
        self.0
    }
}

impl AsRef<[u8]> for Salt {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; SALT_SIZE]> for Salt {
    fn from(bytes: [u8; SALT_SIZE]) -> Self {
        Self(bytes)
    }
}

impl From<Salt> for [u8; SALT_SIZE] {
    fn from(salt: Salt) -> Self {
        salt.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salt_new() {
        let salt = Salt::new([0u8; SALT_SIZE]);
        assert_eq!(salt.as_ref(), [0u8; SALT_SIZE]);
    }

    #[test]
    fn test_salt_as_ref() {
        let salt = Salt::new([0u8; SALT_SIZE]);
        assert_eq!(salt.as_ref(), [0u8; SALT_SIZE]);
    }

    #[test]
    fn test_salt_into_bytes() {
        let salt = Salt::new([0u8; SALT_SIZE]);
        assert_eq!(salt.into_bytes(), [0u8; SALT_SIZE]);
    }

    #[test]
    fn test_salt_from_bytes() {
        let salt = Salt::from([0u8; SALT_SIZE]);
        assert_eq!(salt.as_ref(), [0u8; SALT_SIZE]);
    }

    #[test]
    fn test_salt_from_salt() {
        let salt = Salt::new([0u8; SALT_SIZE]);
        let salt2 = Salt::from(salt);
        assert_eq!(salt2.as_ref(), [0u8; SALT_SIZE]);
    }
}
