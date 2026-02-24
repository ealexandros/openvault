use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::compression::{Compressor, zstd};
use crate::errors::{Error, Result};

pub type CompressorRef = &'static dyn Compressor;

#[repr(u8)]
#[derive(
    Debug, PartialEq, EnumString, Display, AsRefStr, Copy, Clone, Serialize, Deserialize, Default,
)]
#[strum(serialize_all = "lowercase")]
pub enum CompressionAlgorithm {
    #[default]
    Zstd,
}

impl CompressionAlgorithm {
    pub fn resolve(&self) -> Result<CompressorRef> {
        match self {
            CompressionAlgorithm::Zstd => Ok(&zstd::Zstd),
        }
    }
}

impl TryFrom<u8> for CompressionAlgorithm {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            x if x == CompressionAlgorithm::Zstd as u8 => Ok(CompressionAlgorithm::Zstd),
            _ => Err(Error::UnsupportedCompressor(value)),
        }
    }
}
