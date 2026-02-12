use chrono::Utc;
use openvault_secrets::SecretEntryPatch;
use openvault_secrets::domain::entry::{EncryptedField, SecretEntry};
use openvault_secrets::domain::totp::TOTP;

fn ef(s: &str) -> EncryptedField {
    EncryptedField::new(s.as_bytes().to_vec())
}

#[test]
fn test_secret_entry_update() {
    let mut entry = SecretEntry::new(
        "service".into(),
        "user1".into(),
        ef("pass1"),
        Some("site1".into()),
        Some("comment1".into()),
        None,
    )
    .unwrap();

    let update = SecretEntryPatch {
        name: Some("service2".into()),
        username: Some("user2".into()),
        password: Some(ef("pass2")),
        website: Some("site2".into()),
        comments: Some("comment2".into()),
        updated_at: Utc::now(),
        totp: Some(Some(
            TOTP::new("JBSWY3DPEHPK3PXP".into(), None, None).unwrap(),
        )),
    };

    entry.patch(update).unwrap();

    assert_eq!(entry.username, "user2");
    assert_eq!(entry.password, ef("pass2"));
    assert_eq!(entry.website, "site2");
    assert_eq!(entry.comments, "comment2");
    assert!(entry.totp.is_some());
}

#[test]
fn test_secret_entry_partial_update() {
    let mut entry = SecretEntry::new(
        "service".into(),
        "user1".into(),
        ef("pass1"),
        None,
        None,
        None,
    )
    .unwrap();

    let update = SecretEntryPatch {
        password: Some(ef("new_pass")),
        ..Default::default()
    };

    entry.patch(update).unwrap();

    assert_eq!(entry.username, "user1");
    assert_eq!(entry.password, ef("new_pass"));
}

#[test]
fn test_secret_entry_remove_totp() {
    let mut entry = SecretEntry::new(
        "service".into(),
        "user".into(),
        ef("pass"),
        None,
        None,
        Some(TOTP::new("JBSWY3DPEHPK3PXP".into(), None, None).unwrap()),
    )
    .unwrap();

    assert!(entry.totp.is_some());

    let update = SecretEntryPatch {
        totp: Some(None),
        ..Default::default()
    };

    entry.patch(update).unwrap();
    assert!(entry.totp.is_none());
}
