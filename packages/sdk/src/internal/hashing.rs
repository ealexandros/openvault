use crc32fast;

pub fn compute_crc(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}
