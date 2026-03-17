use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::derived_key::DerivedKey;

use super::error::Result;
use super::models::{EncryptedField, EncryptedTotp, TOTP};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginEntryPatch {
    pub folder_id: Option<Uuid>,
    pub name: Option<String>,
    pub username: Option<EncryptedField>,
    pub password: Option<EncryptedField>,
    pub website: Option<EncryptedField>,
    pub comments: Option<EncryptedField>,
    pub totp: Option<Option<EncryptedTotp>>,
    pub updated_at: DateTime<Utc>,
}

impl Default for LoginEntryPatch {
    fn default() -> Self {
        Self {
            folder_id: None,
            name: None,
            username: None,
            password: None,
            website: None,
            comments: None,
            totp: None,
            updated_at: Utc::now(),
        }
    }
}

impl LoginEntryPatch {
    pub fn rename(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn move_to(folder_id: Uuid) -> Self {
        Self {
            folder_id: Some(folder_id),
            ..Default::default()
        }
    }

    pub fn from_plaintext(
        folder_id: Option<Uuid>,
        name: Option<String>,
        username: Option<String>,
        password: Option<String>,
        website: Option<String>,
        comments: Option<String>,
        totp: Option<Option<TOTP>>,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        Ok(Self {
            folder_id,
            name,
            username: username
                .map(|value| EncryptedField::seal_string(value, key, cipher))
                .transpose()?,
            password: password
                .map(|value| EncryptedField::seal_string(value, key, cipher))
                .transpose()?,
            website: website
                .map(|value| EncryptedField::seal_string(value, key, cipher))
                .transpose()?,
            comments: comments
                .map(|value| EncryptedField::seal_string(value, key, cipher))
                .transpose()?,
            totp: match totp {
                Some(Some(value)) => Some(Some(EncryptedTotp::seal(&value, key, cipher)?)),
                Some(None) => Some(None),
                None => None,
            },
            updated_at: Utc::now(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecretFolderPatch {
    pub name: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl Default for SecretFolderPatch {
    fn default() -> Self {
        Self {
            name: None,
            updated_at: Utc::now(),
        }
    }
}

impl SecretFolderPatch {
    pub fn rename(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

impl Zeroize for LoginEntryPatch {
    fn zeroize(&mut self) {
        if let Some(name) = &mut self.name {
            name.zeroize();
        }
        if let Some(username) = &mut self.username {
            username.zeroize();
        }
        if let Some(password) = &mut self.password {
            password.zeroize();
        }
        if let Some(website) = &mut self.website {
            website.zeroize();
        }
        if let Some(comments) = &mut self.comments {
            comments.zeroize();
        }
        if let Some(totp) = &mut self.totp {
            totp.zeroize();
        }
    }
}

impl Zeroize for SecretFolderPatch {
    fn zeroize(&mut self) {
        if let Some(name) = &mut self.name {
            name.zeroize();
        }
    }
}
