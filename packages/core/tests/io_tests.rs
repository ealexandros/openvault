use openvault_core::utils::io::ReadExt;
use std::io::Cursor;

#[test]
fn read_exact_array_ok() {
    let data = [1u8, 2, 3, 4, 5];
    let mut reader = Cursor::new(data);

    let result: [u8; 3] = reader.read_exact_array().unwrap();
    assert_eq!(result, [1, 2, 3]);

    let result: [u8; 2] = reader.read_exact_array().unwrap();
    assert_eq!(result, [4, 5]);
}

#[test]
fn read_exact_array_eof() {
    let data = [1u8, 2];
    let mut reader = Cursor::new(data);

    let result: Result<[u8; 3], _> = reader.read_exact_array();
    assert!(result.is_err());
}
