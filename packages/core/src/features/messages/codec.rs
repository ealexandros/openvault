use super::error::{MessagesError, Result};
use super::events::MessagesChange;
use super::MessagesSnapshot;
use crate::features::shared::FeatureCodec;

pub const MESSAGES_WIRE_VERSION: u16 = 1;

#[derive(Debug, Default, Clone, Copy)]
pub struct MessagesCodec;

impl FeatureCodec for MessagesCodec {
    type Error = MessagesError;
    type DomainChange = MessagesChange;
    type DomainSnapshot = MessagesSnapshot;

    fn wire_version() -> u16 {
        MESSAGES_WIRE_VERSION
    }

    fn encode_change(change: Self::DomainChange) -> Result<Vec<u8>> {
        let payload = postcard::to_allocvec(&change)
            .map_err(|e| MessagesError::InvalidPayload(e.to_string()))?;

        Ok(payload)
    }

    fn decode_change(wire_version: u16, payload: &[u8]) -> Result<Self::DomainChange> {
        if wire_version != MESSAGES_WIRE_VERSION {
            return Err(MessagesError::UnsupportedWireVersion(wire_version));
        }

        let decoded: Self::DomainChange = postcard::from_bytes(payload)
            .map_err(|e| MessagesError::InvalidPayload(e.to_string()))?;

        Ok(decoded)
    }
}
