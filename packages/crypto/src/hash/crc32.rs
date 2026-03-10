use crate::hash::Hasher;

pub struct Crc32Hasher;

impl Crc32Hasher {
    pub fn checksum(data: &[u8]) -> u32 {
        crc32fast::hash(data)
    }
}

impl Hasher for Crc32Hasher {
    fn hash(data: &[u8]) -> [u8; 32] {
        let crc = Crc32Hasher::checksum(data);

        let mut result = [0u8; 32];
        result[..4].copy_from_slice(&crc.to_be_bytes());
        result
    }
}
