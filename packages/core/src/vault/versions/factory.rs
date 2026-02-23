use crate::errors::{Error, Result};
use crate::vault::versions::shared::traits::VersionHandler;
use crate::vault::versions::v1::{V1_FORMAT_VERSION, V1Handler};

pub const LATEST_VERSION: u16 = V1_FORMAT_VERSION;

pub type EngineRef = &'static dyn VersionHandler;

// @todo-now rename the engine to something else

pub fn resolve_engine(version: u16) -> Result<EngineRef> {
    match version {
        V1_FORMAT_VERSION => Ok(&V1Handler),
        _ => Err(Error::UnsupportedVaultVersion(version)),
    }
}

pub fn latest_engine() -> EngineRef {
    &V1Handler
}
