use std::io::Cursor;
use vault_core::crypto::compression::Compressor;
use vault_core::crypto::compression::zstd::Zstd;

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
