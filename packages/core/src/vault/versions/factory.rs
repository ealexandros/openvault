use crate::errors::{Error, Result};
use crate::vault::versions::shared::traits::VersionHandler;
use crate::vault::versions::v1::V1_FORMAT_VERSION;
use crate::vault::versions::v1::V1Handler;

pub const LATEST_VERSION: u16 = V1_FORMAT_VERSION;

pub fn resolve(version: u16) -> Result<Box<dyn VersionHandler>> {
    match version {
        V1_FORMAT_VERSION => Ok(Box::new(V1Handler)),
        _ => Err(Error::UnsupportedVaultVersion(version)),
    }
}

pub fn resolve_latest() -> Result<Box<dyn VersionHandler>> {
    resolve(LATEST_VERSION)
}
