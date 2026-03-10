use zeroize::{ZeroizeOnDrop, Zeroizing};

pub const DK_SIZE: usize = 32;

#[derive(PartialEq, ZeroizeOnDrop)]
pub struct DerivedKey<const N: usize = DK_SIZE>(Zeroizing<[u8; N]>);

impl<const N: usize> DerivedKey<N> {
    pub fn new(bytes: [u8; N]) -> Self {
        Self(Zeroizing::new(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const N: usize> AsRef<[u8]> for DerivedKey<N> {
    fn as_ref(&self) -> &[u8] {
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
