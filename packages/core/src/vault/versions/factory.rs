use crate::errors::{Error, Result};
use crate::vault::versions::shared::traits::VersionHandler;
use crate::vault::versions::v1::V1Handler;

// @todo-now fix boxing everywhere

pub const LATEST_VERSION: u16 = 1;

pub fn resolve(version: u16) -> Result<Box<dyn VersionHandler>> {
    match version {
        1 => Ok(Box::new(V1Handler)),
        _ => Err(Error::UnsupportedVaultVersion(version)),
    }
}

pub fn resolve_latest() -> Result<Box<dyn VersionHandler>> {
    resolve(LATEST_VERSION)
}
