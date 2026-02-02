use openvault_core::utils::crc::{compute_crc, verify_crc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Test {
    text: String,
    crc: u32,
}

#[test]
fn test_crc32() {
    let data = b"Hey Vault!";
    let crc = compute_crc(data);

    assert!(verify_crc(data, crc));
    assert!(!verify_crc(data, crc + 1));
}

#[test]
fn test_crc32_a_struct() {
    let text = "Hey Vault!".to_string();
    let crc = compute_crc(text.as_bytes());

    let data = Test { text, crc };

    let serialized = serde_json::to_vec(&data).unwrap();
    let crc = compute_crc(&serialized);

    println!("serialized: {:?}", serde_json::to_string(&data));

    assert!(verify_crc(&serialized, crc));
    assert!(!verify_crc(&serialized, crc + 1));
}
