use uuid::Uuid;

use crate::errors::{Error, Result};

pub fn parse_uuid(id: &str) -> Result<Uuid> {
    Uuid::parse_str(id).map_err(|_| Error::InvalidUuid(id.to_string()))
}
