use std::ops::Deref;

use zeroize::Zeroizing;

pub const DK_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedKey<const N: usize = DK_SIZE>(Zeroizing<[u8; N]>);

impl<const N: usize> DerivedKey<N> {
    pub fn new(bytes: [u8; N]) -> Self {
        Self(Zeroizing::new(bytes))
    }
}

impl<const N: usize> AsRef<[u8]> for DerivedKey<N> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const N: usize> Deref for DerivedKey<N> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_new() {
        let key = DerivedKey::<32>::new([0u8; 32]);
        assert_eq!(key.as_ref(), [0u8; 32]);
    }

    #[test]
    fn test_derive_key_as_ref() {
        let key = DerivedKey::<32>::new([0u8; 32]);
        assert_eq!(key.as_ref(), [0u8; 32]);
    }
}
