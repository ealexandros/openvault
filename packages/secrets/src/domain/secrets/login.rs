use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

use crate::domain::folders::path::normalize_folder_path;
use crate::domain::secrets::crypto::EncryptedField;
use crate::domain::secrets::totp::TOTP;
use crate::errors::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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
