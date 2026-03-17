use serde::{Deserialize, Serialize};
use std::num::{NonZeroU8, NonZeroU64};
use validator::Validate;
use zeroize::{Zeroize, ZeroizeOnDrop};

use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::derived_key::DerivedKey;

use super::super::error::{Result, SecretError};
use super::sealed_value::SealedValue;

const DEFAULT_PERIOD: NonZeroU64 = NonZeroU64::new(30).unwrap();
const DEFAULT_DIGITS: NonZeroU8 = NonZeroU8::new(6).unwrap();

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate, Zeroize, ZeroizeOnDrop)]
pub struct TOTP {
    #[validate(length(min = 16, message = "Secret must be at least 16 characters"))]
    pub secret: String,
    pub period: NonZeroU64,
    pub digits: NonZeroU8,
}

impl TOTP {
    pub fn new(secret: String, period: Option<u64>, digits: Option<u8>) -> Result<Self> {
        let period = match period {
            Some(p) => NonZeroU64::new(p).ok_or_else(|| {
                SecretError::InvalidInput("TOTP period must be non-zero".to_string())
            })?,
            None => DEFAULT_PERIOD,
        };
        let digits = match digits {
            Some(d) => NonZeroU8::new(d).ok_or_else(|| {
                SecretError::InvalidInput("TOTP digits must be non-zero".to_string())
            })?,
            None => DEFAULT_DIGITS,
        };

        let totp = Self {
            secret,
            period,
            digits,
        };

        totp.validate()
            .map_err(|e| SecretError::InvalidInput(e.to_string()))?;

        Ok(totp)
    }
}

impl Default for TOTP {
    fn default() -> Self {
        Self {
            secret: String::new(),
            period: DEFAULT_PERIOD,
            digits: DEFAULT_DIGITS,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct EncryptedTotp {
    pub secret: SealedValue,
    pub period: NonZeroU64,
    pub digits: NonZeroU8,
}

impl EncryptedTotp {
    pub fn seal(totp: &TOTP, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<Self> {
        Ok(Self {
            secret: SealedValue::seal_string(&totp.secret, key, cipher)?,
            period: totp.period,
            digits: totp.digits,
        })
    }

    pub fn reveal(&self, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<TOTP> {
        let secret = self.secret.reveal_string(key, cipher)?;
        TOTP::new(secret, Some(self.period.get()), Some(self.digits.get()))
    }
}
