use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::errors::{Error, Result};
use crate::internal::secure_memory::SecureMemory;

pub const DK_SIZE: usize = 32;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DerivedKey<const N: usize = DK_SIZE> {
    key: SecureMemory<N>,
}

impl<const N: usize> DerivedKey<N> {
    pub fn new(bytes: [u8; N]) -> Result<Self> {
        let key = SecureMemory::new(bytes).map_err(|_| Error::MemoryLockFailed)?;
        Ok(Self { key })
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.key.as_ref()
    }
}

impl<const N: usize> PartialEq for DerivedKey<N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<const N: usize> AsRef<[u8]> for DerivedKey<N> {
    fn as_ref(&self) -> &[u8] {
        self.key.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_new() {
        let key = DerivedKey::<32>::new([0u8; 32]).unwrap();
        assert_eq!(key.as_ref(), [0u8; 32]);
    }

    #[test]
    fn test_derive_key_as_ref() {
        let key = DerivedKey::<32>::new([0u8; 32]).unwrap();
        assert_eq!(key.as_ref(), [0u8; 32]);
    }
}
