use chrono::Utc;
use openvault_crypto::keys::MasterKey;
use validator::Validate;

use crate::SecretEntry;
use crate::domain::entry::SecretEntryPatch;
use crate::domain::totp::TOTP;
use crate::errors::{Result, SecretError};
use crate::manager::{EncryptedField, codec};

#[derive(Debug, Validate)]
pub struct AddSecretEntryParams {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
    #[validate(length(max = 255))]
    pub website: String,
    #[validate(length(max = 255))]
    pub comments: String,
    pub totp: Option<TOTP>,
}

#[derive(Debug, Validate)]
pub struct UpdateSecretEntryParams {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: Option<String>,
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: Option<String>,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: Option<String>,
    #[validate(length(max = 255))]
    pub website: Option<String>,
    #[validate(length(max = 255))]
    pub comments: Option<String>,
    pub totp: Option<Option<TOTP>>,
}

impl AddSecretEntryParams {
    pub fn into_entry(self, key: &MasterKey) -> Result<SecretEntry> {
        self.validate()
            .map_err(|e| SecretError::InvalidInput(e.to_string()))?;

        let encrypted = codec::encrypt_password(self.password.as_bytes(), key)?;
        let password = EncryptedField::new(encrypted.into_bytes());

        SecretEntry::new(
            self.name,
            self.username,
            password,
            Some(self.website),
            Some(self.comments),
            self.totp,
        )
    }
}

impl UpdateSecretEntryParams {
    pub fn into_patch(self, key: &MasterKey) -> Result<SecretEntryPatch> {
        self.validate()
            .map_err(|e| SecretError::InvalidInput(e.to_string()))?;

        let password = if let Some(p) = self.password {
            let encrypted = codec::encrypt_password(p.as_bytes(), key)?;
            Some(EncryptedField::new(encrypted.into_bytes()))
        } else {
            None
        };

        Ok(SecretEntryPatch {
            name: self.name,
            username: self.username,
            password,
            website: self.website,
            comments: self.comments,
            updated_at: Utc::now(),
            totp: self.totp,
        })
    }
}
