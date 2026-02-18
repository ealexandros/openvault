use crate::errors::{Error, Result};
use crate::vault::versions::shared::traits::VersionHandler;
use crate::vault::versions::v1::V1Handler;

pub const LATEST_VERSION: u16 = 1;

// @todo-now fix boxing everywhere

pub fn get_handler(version: u16) -> Result<Box<dyn VersionHandler>> {
    match version {
        1 => Ok(Box::new(V1Handler)),
        _ => Err(Error::UnsupportedVaultVersion(version)),
    }
}
