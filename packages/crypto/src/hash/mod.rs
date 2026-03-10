pub mod crc32;
pub mod sha256;

pub use crc32::Crc32Hasher;
pub use sha256::Sha256Hasher;

pub trait Hasher {
    fn hash(data: &[u8]) -> [u8; 32];

    fn verify(data: &[u8], expected_hash: &[u8; 32]) -> bool {
        Self::hash(data) == *expected_hash
    }
}
