use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use zeroize::Zeroize;

use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::derived_key::DerivedKey;

use super::super::error::Result;
use super::super::patch::LoginEntryPatch;
use super::{EncryptedTotp, SealedValue, TOTP};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct LoginEntry {
    pub id: Uuid,
    pub folder_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    #[validate(custom(function = "super::super::validate::validate_safe_name"))]
    pub name: String,
    pub username: SealedValue,
    pub password: SealedValue,
    pub website: SealedValue,
    pub comments: SealedValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub totp: Option<EncryptedTotp>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginEntryView {
    pub id: Uuid,
    pub folder_id: Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
    pub website: String,
    pub comments: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub totp: Option<TOTP>,
}

pub struct NewLoginSecret {
    pub folder_id: Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
    pub website: Option<String>,
    pub comments: Option<String>,
    pub totp: Option<TOTP>,
}

impl LoginEntry {
    pub fn seal(
        input: NewLoginSecret,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            folder_id: input.folder_id,
            name: input.name,
            username: SealedValue::seal_string(input.username, key, cipher)?,
            password: SealedValue::seal_string(input.password, key, cipher)?,
            website: SealedValue::seal_string(input.website.unwrap_or_default(), key, cipher)?,
            comments: SealedValue::seal_string(input.comments.unwrap_or_default(), key, cipher)?,
            created_at: now,
            updated_at: now,
            totp: input
                .totp
                .as_ref()
                .map(|v| EncryptedTotp::seal(v, key, cipher))
                .transpose()?,
        })
    }

    pub fn reveal(&self, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<LoginEntryView> {
        Ok(LoginEntryView {
            id: self.id,
            folder_id: self.folder_id,
            name: self.name.clone(),
            username: self.username.reveal_string(key, cipher)?,
            password: self.password.reveal_string(key, cipher)?,
            website: self.website.reveal_string(key, cipher)?,
            comments: self.comments.reveal_string(key, cipher)?,
            created_at: self.created_at,
            updated_at: self.updated_at,
            totp: self
                .totp
                .as_ref()
                .map(|value| value.reveal(key, cipher))
                .transpose()?,
        })
    }

    pub fn apply_patch(&mut self, update: LoginEntryPatch) -> Result {
        if let Some(folder_id) = update.folder_id {
            self.folder_id = folder_id;
        }
        if let Some(name) = update.name {
            self.name = name;
        }
        if let Some(username) = update.username {
            self.username = username;
        }
        if let Some(password) = update.password {
            self.password = password;
        }
        if let Some(website) = update.website {
            self.website = website;
        }
        if let Some(comments) = update.comments {
            self.comments = comments;
        }
        if let Some(totp) = update.totp {
            self.totp = totp;
        }

        self.updated_at = update.updated_at;

        Ok(())
    }
}

impl Zeroize for LoginEntry {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.username.zeroize();
        self.password.zeroize();
        self.website.zeroize();
        self.comments.zeroize();
        self.created_at = DateTime::default();
        self.updated_at = DateTime::default();
        self.totp.zeroize();
    }
}

impl Drop for LoginEntry {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl Zeroize for LoginEntryView {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.username.zeroize();
        self.password.zeroize();
        self.website.zeroize();
        self.comments.zeroize();
        self.created_at = DateTime::default();
        self.updated_at = DateTime::default();
        self.totp.zeroize();
    }
}

impl Drop for LoginEntryView {
    fn drop(&mut self) {
        self.zeroize();
    }
}
