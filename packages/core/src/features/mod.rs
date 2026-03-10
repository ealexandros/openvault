pub mod filesystem;
pub mod messages;
pub mod shared;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::errors::{Error, Result};
use crate::operations::compact::{CompactionBundle, build_bundle_for};
use crate::repositories::{FilesystemRepository, MessagesRepository};
use crate::vault::runtime::VaultSession;

#[repr(u16)]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureType {
    Filesystem = 1,
    Messages = 2,
}

impl FeatureType {
    pub(crate) fn build_bundle(&self, session: &mut VaultSession) -> Result<CompactionBundle> {
        match self {
            FeatureType::Filesystem => build_bundle_for::<FilesystemRepository>(session, *self),
            FeatureType::Messages => build_bundle_for::<MessagesRepository>(session, *self),
        }
    }
}

impl TryFrom<u16> for FeatureType {
    type Error = Error;

    fn try_from(v: u16) -> Result<Self> {
        match v {
            1 => Ok(Self::Filesystem),
            2 => Ok(Self::Messages),
            _ => Err(Error::InvalidVaultFormat),
        }
    }
}
