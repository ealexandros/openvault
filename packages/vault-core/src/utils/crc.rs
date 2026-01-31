use crc32fast::Hasher;

pub fn compute_crc(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

pub fn verify_crc(data: &[u8], crc: u32) -> bool {
    compute_crc(data) == crc
}
