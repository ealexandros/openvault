use openvault_secrets::domain::secrets::totp::TOTP;
use std::num::{NonZeroU8, NonZeroU64};

#[test]
fn test_totp_creation() {
    let secret = "JBSWY3DPEHPK3PXP".to_string();
    let totp = TOTP::new(secret.clone(), Some(60), Some(8)).unwrap();

    assert_eq!(totp.secret, secret);
    assert_eq!(totp.period, NonZeroU64::new(60).unwrap());
    assert_eq!(totp.digits, NonZeroU8::new(8).unwrap());
}

#[test]
fn test_totp_defaults() {
    let secret = "JBSWY3DPEHPK3PXP".to_string();
    let totp = TOTP::new(secret.clone(), None, None).unwrap();

    assert_eq!(totp.period, NonZeroU64::new(30).unwrap());
    assert_eq!(totp.digits, NonZeroU8::new(6).unwrap());
}

#[test]
fn test_totp_validation() {
    let secret = "TOO_SHORT".to_string();
    let result = TOTP::new(secret, None, None);
    assert!(result.is_err());
}

#[test]
fn test_totp_default_trait() {
    let totp = TOTP::default();
    assert_eq!(totp.secret, "");
    assert_eq!(totp.period, NonZeroU64::new(30).unwrap());
    assert_eq!(totp.digits, NonZeroU8::new(6).unwrap());
}
