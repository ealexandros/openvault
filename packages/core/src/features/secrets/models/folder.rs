use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use zeroize::Zeroize;

pub const SECRETS_ROOT_FOLDER_ID: Uuid = Uuid::nil();
pub const SECRETS_ROOT_FOLDER_NAME: &str = "/";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct SecretFolder {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    #[validate(length(min = 1, max = 255))]
    #[validate(custom(function = "super::super::validate::validate_safe_name"))]
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
