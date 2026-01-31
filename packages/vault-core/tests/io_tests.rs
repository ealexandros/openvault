use vault_core::utils::io::{read_u64_le, write_u64_le};

#[test]
fn test_read_write_u64() {
    let mut data = vec![];

    write_u64_le(&mut data, 0x1234567890ABCDEF).unwrap();

    let mut cursor = std::io::Cursor::new(data);
    let val = read_u64_le(&mut cursor).unwrap();

    assert_eq!(val, 0x1234567890ABCDEF);
}
