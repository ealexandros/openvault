use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use zeroize::Zeroize;

use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::derived_key::DerivedKey;

use super::super::error::Result;
use super::super::patch::ApiKeyEntryPatch;
use super::EncryptedField;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct ApiKeyEntry {
    pub id: Uuid,
    pub folder_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    #[validate(custom(function = "super::super::validate::validate_safe_name"))]
    pub name: String,
    pub api_key: EncryptedField,
    pub website: EncryptedField,
    pub comments: EncryptedField,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiKeyEntryView {
    pub id: Uuid,
    pub folder_id: Uuid,
    pub name: String,
    pub api_key: String,
    pub website: String,
    pub comments: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiKeyEntry {
    pub fn seal(
        folder_id: Uuid,
        name: String,
        api_key: String,
        website: Option<String>,
        comments: Option<String>,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            folder_id,
            name,
            api_key: EncryptedField::seal_string(api_key, key, cipher)?,
            website: EncryptedField::seal_string(website.unwrap_or_default(), key, cipher)?,
            comments: EncryptedField::seal_string(comments.unwrap_or_default(), key, cipher)?,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reveal(&self, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<ApiKeyEntryView> {
        Ok(ApiKeyEntryView {
            id: self.id,
            folder_id: self.folder_id,
            name: self.name.clone(),
            api_key: self.api_key.reveal_string(key, cipher)?,
            website: self.website.reveal_string(key, cipher)?,
            comments: self.comments.reveal_string(key, cipher)?,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }

    pub fn apply_patch(&mut self, update: ApiKeyEntryPatch) -> Result {
        if let Some(folder_id) = update.folder_id {
            self.folder_id = folder_id;
        }
        if let Some(name) = update.name {
            self.name = name;
        }
        if let Some(api_key) = update.api_key {
            self.api_key = api_key;
        }
        if let Some(website) = update.website {
            self.website = website;
        }
        if let Some(comments) = update.comments {
            self.comments = comments;
        }

        self.updated_at = update.updated_at;

        Ok(())
    }
}

impl Zeroize for ApiKeyEntry {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.api_key.zeroize();
        self.website.zeroize();
        self.comments.zeroize();
        self.created_at = DateTime::default();
        self.updated_at = DateTime::default();
    }
}

impl Drop for ApiKeyEntry {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl Zeroize for ApiKeyEntryView {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.api_key.zeroize();
        self.website.zeroize();
        self.comments.zeroize();
        self.created_at = DateTime::default();
        self.updated_at = DateTime::default();
    }
}

impl Drop for ApiKeyEntryView {
    fn drop(&mut self) {
        self.zeroize();
    }
}
