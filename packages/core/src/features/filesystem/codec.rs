use crate::features::shared::codec::FeatureCodec;

use super::errors::{FilesystemError, Result};
use super::events::FilesystemChange;

pub const FILESYSTEM_WIRE_VERSION: u16 = 1;

#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemCodec;

impl FeatureCodec for FilesystemCodec {
    type Error = FilesystemError;
    type DomainChange = FilesystemChange;

    fn wire_version(&self) -> u16 {
        FILESYSTEM_WIRE_VERSION
    }

    fn encode_change(&self, change: Self::DomainChange) -> Result<Vec<u8>> {
        let payload = postcard::to_allocvec(&change)
            .map_err(|e| FilesystemError::InvalidPayload(e.to_string()))?;

        Ok(payload)
    }

    fn decode_change(&self, wire_version: u16, payload: &[u8]) -> Result<Self::DomainChange> {
        if wire_version != FILESYSTEM_WIRE_VERSION {
            return Err(FilesystemError::UnsupportedWireVersion(wire_version));
        }

        let decoded: Self::DomainChange = postcard::from_bytes(payload)
            .map_err(|e| FilesystemError::InvalidPayload(e.to_string()))?;

        Ok(decoded)
    }
}
