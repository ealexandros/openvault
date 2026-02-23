use crate::errors::{Error, Result};
use crate::vault::versions::shared::traits::FormatHandler;
use crate::vault::versions::v1::{V1_FORMAT_VERSION, V1FormatHandler};

pub const LATEST_FORMAT_VERSION: u16 = V1_FORMAT_VERSION;

pub type FormatRef = &'static dyn FormatHandler;

pub fn resolve_format(version: u16) -> Result<FormatRef> {
    match version {
        V1_FORMAT_VERSION => Ok(&V1FormatHandler),
        _ => Err(Error::UnsupportedVaultVersion(version)),
    }
}

pub fn latest_format() -> FormatRef {
    &V1FormatHandler
}
