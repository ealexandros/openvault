use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::num::{NonZeroU8, NonZeroU64};
use uuid::Uuid;
use validator::Validate;
use zeroize::{Zeroize, ZeroizeOnDrop};

use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::derived_key::DerivedKey;

use super::error::{Result, SecretError};
use super::patch::LoginEntryPatch;

pub const SECRETS_ROOT_FOLDER_ID: Uuid = Uuid::nil();
pub const SECRETS_ROOT_FOLDER_NAME: &str = "/";

#[derive(Clone, Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, PartialEq, Eq)]
pub struct EncryptedField(Vec<u8>);

impl EncryptedField {
    pub const fn new(ciphertext: Vec<u8>) -> Self {
        Self(ciphertext)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn seal_bytes(
        plaintext: &[u8],
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        let encrypted = cipher
            .resolve()
            .encrypt_prefixed_nonce(key.as_bytes(), plaintext, b"")
            .map_err(|e| SecretError::CryptoError(e.to_string()))?;

        Ok(Self(encrypted))
    }

    pub fn reveal_bytes(&self, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<Vec<u8>> {
        cipher
            .resolve()
            .decrypt_prefixed_nonce(key.as_bytes(), &self.0, b"")
            .map_err(|e| SecretError::CryptoError(e.to_string()))
    }

    pub fn seal_string(
        plaintext: impl AsRef<str>,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        Self::seal_bytes(plaintext.as_ref().as_bytes(), key, cipher)
    }

    pub fn reveal_string(&self, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<String> {
        let bytes = self.reveal_bytes(key, cipher)?;
        String::from_utf8(bytes).map_err(|e| SecretError::InvalidInput(e.to_string()))
    }

    pub fn seal_value<T: Serialize>(
        value: &T,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        let payload = postcard::to_allocvec(value)
            .map_err(|e| SecretError::SerializationError(e.to_string()))?;
        Self::seal_bytes(&payload, key, cipher)
    }

    pub fn reveal_value<T: DeserializeOwned>(
        &self,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<T> {
        let payload = self.reveal_bytes(key, cipher)?;
        postcard::from_bytes(&payload).map_err(|e| SecretError::DeserializationError(e.to_string()))
    }
}

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
    pub secret: EncryptedField,
    pub period: NonZeroU64,
    pub digits: NonZeroU8,
}

impl EncryptedTotp {
    pub fn seal(totp: &TOTP, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<Self> {
        Ok(Self {
            secret: EncryptedField::seal_string(&totp.secret, key, cipher)?,
            period: totp.period,
            digits: totp.digits,
        })
    }

    pub fn reveal(&self, key: &DerivedKey, cipher: EncryptionAlgorithm) -> Result<TOTP> {
        let secret = self.secret.reveal_string(key, cipher)?;
        TOTP::new(secret, Some(self.period.get()), Some(self.digits.get()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct SecretFolder {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    #[validate(length(min = 1, max = 255))]
    #[validate(custom(function = "super::validate::validate_safe_name"))]
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SecretFolder {
    pub fn new(parent_id: Option<Uuid>, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            parent_id,
            name: name.into(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn root() -> Self {
        let now = Utc::now();
        Self {
            id: SECRETS_ROOT_FOLDER_ID,
            parent_id: None,
            name: SECRETS_ROOT_FOLDER_NAME.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Zeroize for SecretFolder {
    fn zeroize(&mut self) {
        self.name.zeroize();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct LoginEntry {
    pub id: Uuid,
    pub folder_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    #[validate(custom(function = "super::validate::validate_safe_name"))]
    pub name: String,
    pub username: EncryptedField,
    pub password: EncryptedField,
    pub website: EncryptedField,
    pub comments: EncryptedField,
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

impl LoginEntry {
    pub fn seal(
        folder_id: Uuid,
        name: String,
        username: String,
        password: String,
        website: Option<String>,
        comments: Option<String>,
        totp: Option<TOTP>,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            folder_id,
            name,
            username: EncryptedField::seal_string(username, key, cipher)?,
            password: EncryptedField::seal_string(password, key, cipher)?,
            website: EncryptedField::seal_string(website.unwrap_or_default(), key, cipher)?,
            comments: EncryptedField::seal_string(comments.unwrap_or_default(), key, cipher)?,
            created_at: now,
            updated_at: now,
            totp: totp
                .as_ref()
                .map(|value| EncryptedTotp::seal(value, key, cipher))
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
