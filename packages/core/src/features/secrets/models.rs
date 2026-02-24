use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::num::{NonZeroU8, NonZeroU64};
use uuid::Uuid;
use validator::Validate;
use zeroize::{Zeroize, ZeroizeOnDrop};

use super::error::{Result, SecretError};

pub const ROOT_FOLDER: &str = "/";

pub fn normalize_folder_path(path: &str) -> String {
    let trimmed = path.trim();

    if trimmed.is_empty() || trimmed == ROOT_FOLDER {
        return ROOT_FOLDER.to_string();
    }

    let parts: Vec<&str> = trimmed
        .split('/')
        .filter(|segment| !segment.trim().is_empty())
        .collect();

    if parts.is_empty() {
        return ROOT_FOLDER.to_string();
    }

    format!("/{}", parts.join("/"))
}

#[derive(Clone, Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, PartialEq, Eq)]
pub struct EncryptedField(Vec<u8>);

impl EncryptedField {
    pub const fn new(ciphertext: Vec<u8>) -> Self {
        Self(ciphertext)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

const DEFAULT_PERIOD: u64 = 30;
const DEFAULT_DIGITS: u8 = 6;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate, Zeroize, ZeroizeOnDrop)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginEntry {
    pub id: Uuid,
    pub folder: String,
    pub name: String,
    pub username: String,
    pub password: EncryptedField,
    pub website: String,
    pub comments: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub totp: Option<TOTP>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginEntryPatch {
    pub folder: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<EncryptedField>,
    pub website: Option<String>,
    pub comments: Option<String>,
    pub totp: Option<Option<TOTP>>,
    pub updated_at: DateTime<Utc>,
}

impl LoginEntry {
    pub fn new(
        folder: Option<String>,
        name: String,
        username: String,
        password: EncryptedField,
        website: Option<String>,
        comments: Option<String>,
        totp: Option<TOTP>,
    ) -> Result<Self> {
        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            folder: normalize_folder_path(folder.as_deref().unwrap_or_default()),
            name,
            username,
            password,
            website: website.unwrap_or_default(),
            comments: comments.unwrap_or_default(),
            created_at: now,
            updated_at: now,
            totp,
        })
    }

    pub fn patch(&mut self, update: LoginEntryPatch) -> Result {
        if let Some(folder) = update.folder {
            self.folder = normalize_folder_path(&folder);
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
        self.folder.zeroize();
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
