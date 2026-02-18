use serde::{Deserialize, Serialize};
use std::num::{NonZeroU8, NonZeroU64};
use validator::Validate;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::errors::{Result, SecretError};

const DEFAULT_PERIOD: u64 = 30;
const DEFAULT_DIGITS: u8 = 6;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Validate, Zeroize, ZeroizeOnDrop)]
pub struct TOTP {
    #[validate(length(min = 16, message = "Secret must be at least 16 characters"))]
    pub secret: String,
    pub period: NonZeroU64,
    pub digits: NonZeroU8,
}

impl TOTP {
    pub fn new(secret: String, period: Option<u64>, digits: Option<u8>) -> Result<Self> {
        let totp = Self {
            secret,
            period: NonZeroU64::new(period.unwrap_or(DEFAULT_PERIOD)).unwrap(),
            digits: NonZeroU8::new(digits.unwrap_or(DEFAULT_DIGITS)).unwrap(),
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
            period: NonZeroU64::new(DEFAULT_PERIOD).unwrap(),
            digits: NonZeroU8::new(DEFAULT_DIGITS).unwrap(),
        }
    }
}
