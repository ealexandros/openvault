use strum_macros::{AsRefStr, Display, EnumString};

use crate::compression::{Compressor, zstd};
use crate::errors::Result;

// @todo-soon rethink about name and Box<...>, consider rust-delegate

pub type CompressorRef = &'static dyn Compressor;

#[derive(Debug, PartialEq, EnumString, Display, AsRefStr, Default)]
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
