use super::error::{Result, SecretError};
use super::records::{SECRETS_WIRE_VERSION, SecretSnapshot, SecretsChange};
use crate::features::shared::FeatureCodec;

pub const SECRETS_FEATURE_ID: &str = "secrets";

#[derive(Debug, Default, Clone, Copy)]
pub struct SecretCodec;

impl FeatureCodec for SecretCodec {
    type Error = SecretError;
    type DomainChange = SecretsChange;
    type DomainSnapshot = SecretSnapshot;

    fn wire_version() -> u16 {
        SECRETS_WIRE_VERSION
    }

    fn encode_change(change: Self::DomainChange) -> Result<Vec<u8>> {
        let payload = postcard::to_allocvec(&change)
            .map_err(|e| SecretError::InvalidPayload(e.to_string()))?;

        Ok(payload)
    }

    fn decode_change(wire_version: u16, payload: &[u8]) -> Result<Self::DomainChange> {
        if wire_version != SECRETS_WIRE_VERSION {
            return Err(SecretError::UnsupportedWireVersion(wire_version));
        }

        let decoded: Self::DomainChange = postcard::from_bytes(payload)
            .map_err(|e| SecretError::InvalidPayload(e.to_string()))?;

        Ok(decoded)
    }
}
