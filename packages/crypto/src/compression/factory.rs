use strum_macros::{AsRefStr, Display, EnumString};

use crate::compression::{Compressor, zstd};
use crate::errors::Result;

pub type CompressorRef = &'static dyn Compressor;

#[derive(Debug, PartialEq, EnumString, Display, AsRefStr, Copy, Clone, Default)]
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
