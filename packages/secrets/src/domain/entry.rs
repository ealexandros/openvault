use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::domain::totp::TOTP;
use crate::errors::Result;

#[derive(Clone, Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, PartialEq)]
pub struct EncryptedField(Vec<u8>);

impl EncryptedField {
    pub fn new(ciphertext: Vec<u8>) -> Self {
        Self(ciphertext)
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretEntry {
    pub name: String,
    pub username: String,
    pub password: EncryptedField,
    pub website: String,
    pub comments: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub totp: Option<TOTP>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretEntryView {
    pub name: String,
    pub username: String,
    pub website: String,
    pub comments: String,
    pub totp: Option<TOTP>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecretEntryPatch {
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<EncryptedField>,
    pub website: Option<String>,
    pub comments: Option<String>,
    pub totp: Option<Option<TOTP>>,
    pub updated_at: DateTime<Utc>,
}

impl SecretEntry {
    pub fn new(
        name: String,
        username: String,
        password: EncryptedField,
        website: Option<String>,
        comments: Option<String>,
        totp: Option<TOTP>,
    ) -> Result<Self> {
        let now = Utc::now();

        Ok(Self {
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

    pub fn patch(&mut self, update: SecretEntryPatch) -> Result {
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

impl Zeroize for SecretEntry {
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

impl Drop for SecretEntry {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl From<&SecretEntry> for SecretEntryView {
    fn from(entry: &SecretEntry) -> Self {
        Self {
            name: entry.name.clone(),
            username: entry.username.clone(),
            website: entry.website.clone(),
            comments: entry.comments.clone(),
            totp: entry.totp.clone(),
        }
    }
}
