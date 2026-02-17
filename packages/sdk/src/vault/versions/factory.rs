use crate::errors::{Error, Result};
use crate::versions::shared::traits::VersionHandler;

pub const LATEST_VERSION: u16 = 1;

// @todo-now implement this

pub fn get_handler(version: u16) -> Result<Box<dyn VersionHandler>> {
    // match version {
    //     1 => Ok(Box::new(v1::V1Handler)),
    //     _ => Err(Error::UnsupportedVaultVersion(version)),
    // }

    Err(Error::UnsupportedVaultVersion(version))
}
