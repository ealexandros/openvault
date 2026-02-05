use super::Compressor;
use crate::errors::{CompressionError, Result};
use std::io::{Cursor, Read, Write};

#[derive(Debug, Default, Clone, Copy)]
pub struct Zstd;

impl Compressor for Zstd {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::stream::encode_all(Cursor::new(data), 0)
            .map_err(|e| CompressionError::Compress(e.to_string()).into())
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::stream::decode_all(Cursor::new(data))
            .map_err(|e| CompressionError::Decompress(e.to_string()).into())
    }

    fn compress_stream(&self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()> {
        zstd::stream::copy_encode(input, output, 0)
            .map_err(|e| CompressionError::Compress(e.to_string()))?;
        Ok(())
    }

    fn decompress_stream(&self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()> {
        zstd::stream::copy_decode(input, output)
            .map_err(|e| CompressionError::Decompress(e.to_string()))?;
        Ok(())
    }
}
