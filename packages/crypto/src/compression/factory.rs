use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::compression::{Compressor, zstd};
use crate::errors::{Error, Result};

pub type CompressorRef = &'static dyn Compressor;

#[repr(u8)]
#[derive(Debug, PartialEq, EnumString, Copy, Clone, Serialize, Deserialize, Default)]
#[strum(serialize_all = "lowercase")]
pub enum CompressionAlgorithm {
    #[default]
    Zstd = 1,
}

impl CompressionAlgorithm {
    pub fn resolve(self) -> CompressorRef {
        match self {
            Self::Zstd => &zstd::Zstd,
        }
    }
}

impl TryFrom<u8> for CompressionAlgorithm {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Self::Zstd),
            _ => Err(Error::UnsupportedCompressor(value)),
        }
    }
}
