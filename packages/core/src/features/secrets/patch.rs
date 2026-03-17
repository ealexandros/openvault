use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

use openvault_crypto::encryption::EncryptionAlgorithm;
use openvault_crypto::keys::derived_key::DerivedKey;

use super::error::Result;
use super::models::{EncryptedTotp, SealedValue, TOTP};

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ApiKeyEntryPatch {
//     pub folder_id: Option<Uuid>,
//     pub name: Option<String>,
//     pub api_key: Option<EncryptedField>,
//     pub website: Option<EncryptedField>,
//     pub comments: Option<EncryptedField>,
//     pub updated_at: DateTime<Utc>,
// }

// impl Default for ApiKeyEntryPatch {
//     fn default() -> Self {
//         Self {
//             folder_id: None,
//             name: None,
//             api_key: None,
//             website: None,
//             comments: None,
//             updated_at: Utc::now(),
//         }
//     }
// }

// impl ApiKeyEntryPatch {
//     pub fn rename(name: impl Into<String>) -> Self {
//         Self {
//             name: Some(name.into()),
//             ..Default::default()
//         }
//     }

//     pub fn move_to(folder_id: Uuid) -> Self {
//         Self {
//             folder_id: Some(folder_id),
//             ..Default::default()
//         }
//     }

//     pub fn from_plaintext(
//         folder_id: Option<Uuid>,
//         name: Option<String>,
//         api_key: Option<String>,
//         website: Option<String>,
//         comments: Option<String>,
//         key: &DerivedKey,
//         cipher: EncryptionAlgorithm,
//     ) -> Result<Self> {
//         Ok(Self {
//             folder_id,
//             name,
//             api_key: api_key
//                 .map(|value| EncryptedField::seal_string(value, key, cipher))
//                 .transpose()?,
//             website: website
//                 .map(|value| EncryptedField::seal_string(value, key, cipher))
//                 .transpose()?,
//             comments: comments
//                 .map(|value| EncryptedField::seal_string(value, key, cipher))
//                 .transpose()?,
//             updated_at: Utc::now(),
//         })
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginEntryPatch {
    pub folder_id: Option<Uuid>,
    pub name: Option<String>,
    pub username: Option<SealedValue>,
    pub password: Option<SealedValue>,
    pub website: Option<SealedValue>,
    pub comments: Option<SealedValue>,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NewLoginSecretPatch {
    pub folder_id: Option<Uuid>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub website: Option<String>,
    pub comments: Option<String>,
    pub totp: Option<Option<TOTP>>,
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
        input: NewLoginSecretPatch,
        key: &DerivedKey,
        cipher: EncryptionAlgorithm,
    ) -> Result<Self> {
        Ok(Self {
            folder_id: input.folder_id,
            name: input.name,
            username: input
                .username
                .map(|value| SealedValue::seal_string(value, key, cipher))
                .transpose()?,
            password: input
                .password
                .map(|value| SealedValue::seal_string(value, key, cipher))
                .transpose()?,
            website: input
                .website
                .map(|value| SealedValue::seal_string(value, key, cipher))
                .transpose()?,
            comments: input
                .comments
                .map(|value| SealedValue::seal_string(value, key, cipher))
                .transpose()?,
            totp: match input.totp {
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

// impl Zeroize for ApiKeyEntryPatch {
//     fn zeroize(&mut self) {
//         if let Some(name) = &mut self.name {
//             name.zeroize();
//         }
//         if let Some(api_key) = &mut self.api_key {
//             api_key.zeroize();
//         }
//         if let Some(website) = &mut self.website {
//             website.zeroize();
//         }
//         if let Some(comments) = &mut self.comments {
//             comments.zeroize();
//         }
//     }
// }

impl Zeroize for SecretFolderPatch {
    fn zeroize(&mut self) {
        if let Some(name) = &mut self.name {
            name.zeroize();
        }
    }
}
