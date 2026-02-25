use crate::features::shared::feature_trait::{EncodedFeatureRecord, FeatureCodec, WireChange};

use super::error::SecretError;
use super::records::{SECRETS_WIRE_VERSION_V1, SecretsChange};

pub const SECRETS_FEATURE_ID: &str = "secrets";

#[derive(Debug, Default, Clone, Copy)]
pub struct SecretCodec;

impl FeatureCodec for SecretCodec {
    type Error = SecretError;
    type DomainChange = SecretsChange;

    fn feature_id(&self) -> &'static str {
        SECRETS_FEATURE_ID
    }

    fn wire_version(&self) -> u16 {
        SECRETS_WIRE_VERSION_V1
    }

    fn encode_change(&self, change: Self::DomainChange) -> Result<Vec<u8>> {
        let payload = postcard::to_allocvec(&change)
            .map_err(|e| SecretError::SerializationError(e.to_string()))?;

        Ok(payload)
    }

    fn decode_change(&self, wire_version: u16, payload: &[u8]) -> Result<Self::DomainChange> {
        if wire_version != SECRETS_WIRE_VERSION_V1 {
            return Err(SecretError::UnsupportedWireVersion(wire_version));
        }

        let decoded: Self::DomainChange = postcard::from_bytes(payload)
            .map_err(|e| SecretError::InvalidPayload(e.to_string()))?;

        Ok(decoded)
    }
}
