use crc32fast::Hasher;

pub fn crc32(value: &str) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(value.as_bytes());
    hasher.finalize()
}
