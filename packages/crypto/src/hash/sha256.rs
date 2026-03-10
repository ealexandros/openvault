use sha2::{Digest, Sha256};

use crate::hash::Hasher;

pub struct Sha256Hasher;

impl Hasher for Sha256Hasher {
    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();

        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        hash_bytes
    }
}
