use serde::{Deserialize, Serialize};

use crate::errors::{Error, Result};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureType {
    Filesystem = 1,
    Secrets = 2,
}

impl TryFrom<u16> for FeatureType {
    type Error = Error;

    fn try_from(v: u16) -> Result<Self> {
        match v {
            1 => Ok(Self::Filesystem),
            2 => Ok(Self::Secrets),
            _ => Err(Error::InvalidVaultFormat),
        }
    }
}
