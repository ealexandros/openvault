use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::errors::{Error, Result};

// @todo-now move this into the features folder

#[repr(u16)]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureType {
    Filesystem = 1,
}

impl TryFrom<u16> for FeatureType {
    type Error = Error;

    fn try_from(v: u16) -> Result<Self> {
        match v {
            1 => Ok(Self::Filesystem),
            _ => Err(Error::InvalidVaultFormat),
        }
    }
}
