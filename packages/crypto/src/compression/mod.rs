use std::fmt::Debug;
use std::io::{Read, Write};

use crate::errors::Result;

pub trait Compressor: Debug + Send + Sync {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn compress_stream(&self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()>;
    fn decompress_stream(&self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()>;
}

pub mod factory;
pub mod zstd;
