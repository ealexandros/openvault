use argon2::password_hash::rand_core::{OsRng, RngCore};

pub fn random_bytes<const N: usize>() -> [u8; N] {
    let mut buffer = [0u8; N];
    OsRng.fill_bytes(&mut buffer);
    buffer
}
