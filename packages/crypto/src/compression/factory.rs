use strum_macros::{AsRefStr, Display, EnumString};

use crate::compression::{Compressor, zstd};
use crate::errors::Result;

#[derive(Debug, PartialEq, EnumString, Display, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum CompressionAlgorithm {
    #[default]
    Zstd,
}

impl CompressionAlgorithm {
    pub fn get(&self) -> Result<Box<dyn Compressor>> {
        match self {
            CompressionAlgorithm::Zstd => Ok(Box::new(zstd::Zstd)),
        }
    }
}
