use super::{Compressor, zstd};
use crate::crypto::Result;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum CompressionAlgorithm {
    Zstd,
}

impl CompressionAlgorithm {
    pub fn get(&self) -> Result<Box<dyn Compressor>> {
        match self {
            CompressionAlgorithm::Zstd => Ok(Box::new(zstd::Zstd)),
        }
    }
}
