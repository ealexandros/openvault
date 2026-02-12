use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub content_type: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
