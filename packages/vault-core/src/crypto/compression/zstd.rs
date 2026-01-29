use super::Compressor;
use crate::crypto::error::{CryptoError, Result};
use std::io::{Cursor, Read, Write};

#[derive(Debug, Default, Clone, Copy)]
pub struct Zstd;

impl Compressor for Zstd {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::stream::encode_all(Cursor::new(data), 0)
            .map_err(|e| CryptoError::Compression(e.to_string()))
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::stream::decode_all(Cursor::new(data))
            .map_err(|e| CryptoError::Decompression(e.to_string()))
    }

    fn compress_stream(&self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()> {
        zstd::stream::copy_encode(input, output, 0)
            .map_err(|e| CryptoError::Compression(e.to_string()))?;
        Ok(())
    }

    fn decompress_stream(&self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()> {
        zstd::stream::copy_decode(input, output)
            .map_err(|e| CryptoError::Decompression(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zstd_roundtrip() {
        let compressor = Zstd;
        let data = b"Hello world! This is a test string for Zstd compression.";

        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_zstd_streaming() {
        let compressor = Zstd;
        let data = b"Large-ish data to test streaming compression functionality.".repeat(1000);

        let mut input = Cursor::new(&data);
        let mut compressed = Vec::new();

        compressor
            .compress_stream(&mut input, &mut compressed)
            .unwrap();

        let mut output = Vec::new();
        let mut compressed_input = Cursor::new(&compressed);

        compressor
            .decompress_stream(&mut compressed_input, &mut output)
            .unwrap();

        assert_eq!(data, output);
    }
}
