use chrono::Utc;
use openvault_crypto::keys::MasterKey;
use validator::Validate;

use crate::domain::secrets::crypto::EncryptedField;
use crate::domain::secrets::login::{LoginEntry, LoginEntryPatch};
use crate::domain::secrets::totp::TOTP;
use crate::errors::{Result, SecretError};
use crate::manager::codec;

#[derive(Debug, Validate)]
pub struct AddSecretEntryParams {
    #[validate(length(max = 255))]
    pub folder: String,
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
    #[validate(length(max = 255))]
    pub folder: Option<String>,
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
    pub fn into_entry(self, key: &MasterKey) -> Result<LoginEntry> {
        self.validate()
            .map_err(|e| SecretError::InvalidInput(e.to_string()))?;

        let encrypted = codec::encrypt_password(self.password.as_bytes(), key)?;
        let password = EncryptedField::new(encrypted);

        LoginEntry::new(
            Some(self.folder),
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
    pub fn into_patch(self, key: &MasterKey) -> Result<LoginEntryPatch> {
        self.validate()
            .map_err(|e| SecretError::InvalidInput(e.to_string()))?;

        let password = if let Some(p) = self.password {
            let encrypted = codec::encrypt_password(p.as_bytes(), key)?;
            Some(EncryptedField::new(encrypted))
        } else {
            None
        };

        Ok(LoginEntryPatch {
            name: self.name,
            folder: self.folder,
            username: self.username,
            password,
            website: self.website,
            comments: self.comments,
            updated_at: Utc::now(),
            totp: self.totp,
        })
    }
}
