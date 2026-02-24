use crate::features::feature_trait::{EncodedFeatureRecord, FeatureCodec, RecordKind};

use super::error::SecretError;
use super::records::{SECRETS_WIRE_VERSION_V1, SecretsChange};

pub const SECRETS_FEATURE_ID: &str = "secrets";

#[derive(Debug, Default, Clone, Copy)]
pub struct SecretCodec;

impl FeatureCodec for SecretCodec {
    type DomainChange = SecretsChange;

    fn feature_id(&self) -> &'static str {
        SECRETS_FEATURE_ID
    }

    fn current_wire_version(&self) -> u16 {
        SECRETS_WIRE_VERSION_V1
    }

    fn encode_change(
        &self,
        change: Self::DomainChange,
    ) -> std::result::Result<EncodedFeatureRecord, String> {
        let kind = match &change {
            SecretsChange::Snapshot(_) => RecordKind::Snapshot,
            SecretsChange::Deltas(_) => RecordKind::Delta,
        };

        let payload = postcard::to_allocvec(&change)
            .map_err(|e| SecretError::SerializationError(e.to_string()).to_string())?;

        Ok(EncodedFeatureRecord {
            feature_id: self.feature_id(),
            version: self.current_wire_version(),
            kind,
            payload,
        })
    }

    fn decode_change(
        &self,
        wire_version: u16,
        kind: RecordKind,
        payload: &[u8],
    ) -> std::result::Result<Self::DomainChange, String> {
        if wire_version != SECRETS_WIRE_VERSION_V1 {
            return Err(SecretError::UnsupportedWireVersion(wire_version).to_string());
        }

        let change: SecretsChange = postcard::from_bytes(payload)
            .map_err(|e| SecretError::DeserializationError(e.to_string()).to_string())?;

        let expected = match &change {
            SecretsChange::Snapshot(_) => RecordKind::Snapshot,
            SecretsChange::Deltas(_) => RecordKind::Delta,
        };

        if kind != expected {
            return Err(SecretError::InvalidRecordKind {
                expected,
                actual: kind,
            }
            .to_string());
        }

        Ok(change)
    }
}
