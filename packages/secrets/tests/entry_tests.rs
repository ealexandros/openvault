// use chrono::Utc;
// use openvault_secrets::LoginEntryPatch;
// use openvault_secrets::domain::secrets::crypto::EncryptedField;
// use openvault_secrets::domain::secrets::login::LoginEntry;
// use openvault_secrets::domain::secrets::totp::TOTP;

// fn ef(s: &str) -> EncryptedField {
//     EncryptedField::new(s.as_bytes().to_vec())
// }

// #[test]
// fn test_secret_entry_update() {
//     let mut entry = LoginEntry::new(
//         Some("personal".into()),
//         "service".into(),
//         "user1".into(),
//         ef("pass1"),
//         Some("site1".into()),
//         Some("comment1".into()),
//         None,
//     )
//     .unwrap();

//     let update = LoginEntryPatch {
//         folder: Some("work".into()),
//         name: Some("service2".into()),
//         username: Some("user2".into()),
//         password: Some(ef("pass2")),
//         website: Some("site2".into()),
//         comments: Some("comment2".into()),
//         updated_at: Utc::now(),
//         totp: Some(Some(
//             TOTP::new("JBSWY3DPEHPK3PXP".into(), None, None).unwrap(),
//         )),
//     };

//     entry.patch(update).unwrap();

//     assert_eq!(entry.username, "user2");
//     assert_eq!(entry.folder, "/work");
//     assert_eq!(entry.password, ef("pass2"));
//     assert_eq!(entry.website, "site2");
//     assert_eq!(entry.comments, "comment2");
//     assert!(entry.totp.is_some());
// }

// #[test]
// fn test_secret_entry_partial_update() {
//     let mut entry = LoginEntry::new(
//         None,
//         "service".into(),
//         "user1".into(),
//         ef("pass1"),
//         None,
//         None,
//         None,
//     )
//     .unwrap();

//     let update = LoginEntryPatch {
//         password: Some(ef("new_pass")),
//         ..Default::default()
//     };

//     entry.patch(update).unwrap();

//     assert_eq!(entry.username, "user1");
//     assert_eq!(entry.password, ef("new_pass"));
// }

// #[test]
// fn test_secret_entry_remove_totp() {
//     let mut entry = LoginEntry::new(
//         None,
//         "service".into(),
//         "user".into(),
//         ef("pass"),
//         None,
//         None,
//         Some(TOTP::new("JBSWY3DPEHPK3PXP".into(), None, None).unwrap()),
//     )
//     .unwrap();

//     assert!(entry.totp.is_some());

//     let update = LoginEntryPatch {
//         totp: Some(None),
//         ..Default::default()
//     };

//     entry.patch(update).unwrap();
//     assert!(entry.totp.is_none());
// }
