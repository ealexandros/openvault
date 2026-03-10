use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::errors::{Error, Result};

pub fn parse_uuid(id: &str) -> Result<Uuid> {
    Uuid::parse_str(id).map_err(|_| Error::InvalidUuid(id.to_string()))
}

pub fn parse_optional_datetime(input: Option<String>) -> Result<Option<DateTime<Utc>>> {
    input
        .map(|s| {
            DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|_| Error::InvalidInput("Invalid expiration date".to_string()))
        })
        .transpose()
}
