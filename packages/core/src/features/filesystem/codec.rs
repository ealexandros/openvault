use crate::features::shared::feature_trait::FeatureCodec;

use super::error::{FilesystemError, Result};
use super::records::{FILESYSTEM_WIRE_VERSION_V1, FilesystemChange};

pub const FILESYSTEM_FEATURE_ID: &str = "filesystem";

#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemCodec;

impl FeatureCodec for FilesystemCodec {
    type Error = FilesystemError;
    type DomainChange = FilesystemChange;

    fn feature_id(&self) -> &'static str {
        FILESYSTEM_FEATURE_ID
    }

    fn wire_version(&self) -> u16 {
        FILESYSTEM_WIRE_VERSION_V1
    }

    fn encode_change(&self, change: Self::DomainChange) -> Result<Vec<u8>> {
        let payload = postcard::to_allocvec(&change)
            .map_err(|e| FilesystemError::InvalidPayload(e.to_string()))?;

        Ok(payload)
    }

    fn decode_change(&self, wire_version: u16, payload: &[u8]) -> Result<Self::DomainChange> {
        if wire_version != FILESYSTEM_WIRE_VERSION_V1 {
            return Err(FilesystemError::UnsupportedWireVersion(wire_version));
        }

        let decoded: Self::DomainChange = postcard::from_bytes(payload)
            .map_err(|e| FilesystemError::InvalidPayload(e.to_string()))?;

        Ok(decoded)
    }
}
