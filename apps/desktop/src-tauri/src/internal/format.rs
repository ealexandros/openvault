use crate::errors::{Error, Result};

pub fn string_from_bytes(bytes: Vec<u8>) -> Result<String> {
    String::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)
}
