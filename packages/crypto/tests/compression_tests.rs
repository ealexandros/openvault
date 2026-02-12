use openvault_crypto::compression::zstd::Zstd;
use openvault_crypto::compression::{CompressionAlgorithm, Compressor};
use std::io::Cursor;
use std::str::FromStr;

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

#[test]
fn test_zstd_empty_input() {
    let compressor = CompressionAlgorithm::Zstd.get().unwrap();
    let data = b"";

    let compressed = compressor.compress(data).unwrap();
    let decompressed = compressor.decompress(&compressed).unwrap();

    assert_eq!(data.to_vec(), decompressed);
}

#[test]
fn test_zstd_invalid_data() {
    let compressor = CompressionAlgorithm::Zstd.get().unwrap();
    let invalid_data = b"not compressed at all";

    let result = compressor.decompress(invalid_data);
    assert!(result.is_err());
}

#[test]
fn test_zstd_large_data_roundtrip() {
    let compressor = CompressionAlgorithm::Zstd.get().unwrap();
    let data = vec![0u8; 1024 * 1024];

    let compressed = compressor.compress(&data).unwrap();
    assert!(compressed.len() < data.len());

    let decompressed = compressor.decompress(&compressed).unwrap();
    assert_eq!(data, decompressed);
}
