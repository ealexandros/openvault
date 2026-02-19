// use openvault_crypto::keys::MasterKey;
// use openvault_secrets::manager::codec;

// #[test]
// fn test_codec_roundtrip() {
//     let key = MasterKey::new([0x48; 32]);
//     let data = b"some secret data to encrypt and compress";

//     let encoded = codec::encrypt(data, &key).unwrap();
//     assert_ne!(encoded, data);

//     let decoded = codec::decrypt(&encoded, &key).unwrap();
//     assert_eq!(decoded, data);
// }

// #[test]
// fn test_codec_wrong_key() {
//     let key1 = MasterKey::new([1u8; 32]);
//     let key2 = MasterKey::new([2u8; 32]);
//     let data = b"top secret";

//     let encoded = codec::encrypt(data, &key1).unwrap();
//     let decoded_result = codec::decrypt(&encoded, &key2);

//     assert!(decoded_result.is_err());
// }

// #[test]
// fn test_codec_empty_data() {
//     let key = MasterKey::new([0u8; 32]);
//     let data = b"";

//     let encoded = codec::encrypt(data, &key).unwrap();
//     let decoded = codec::decrypt(&encoded, &key).unwrap();

//     assert_eq!(decoded, data);
// }
