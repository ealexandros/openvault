use std::io::Cursor;
use std::str::FromStr;
use vault_core::crypto::compression::zstd::Zstd;
use vault_core::crypto::compression::{Compressor, factory::CompressionAlgorithm};

#[test]
fn test_zstd_roundtrip() {
    let compressor = CompressionAlgorithm::Zstd.get().unwrap();
    let data = b"Hello world! This is a test string for Zstd compression.";

    let compressed = compressor.compress(data).unwrap();
    let decompressed = compressor.decompress(&compressed).unwrap();

    assert_eq!(data.to_vec(), decompressed);
}

#[test]
fn test_compression_factory_from_str() {
    let algo = CompressionAlgorithm::from_str("zstd").unwrap();
    assert_eq!(algo, CompressionAlgorithm::Zstd);

    let compressor = algo.get().unwrap();
    assert!(compressor.compress(b"test").is_ok());
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
