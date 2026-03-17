use std::collections::HashMap;

use crate::errors::Result;
use crate::features::FeatureType;
use crate::features::secrets::{SecretCodec, SecretStore, SecretsChange};
use crate::features::shared::{BlobRef, FeatureCodec};
use crate::operations::history::append_record;
use crate::operations::replay::replay_since_checkpoint;
use crate::repositories::{CommitOutcome, FeatureRepository};
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::checkpoint::CheckpointFeature;
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::replay::ReplayState;

pub struct SecretsRepository;

impl FeatureRepository for SecretsRepository {
    type Store = SecretStore;
    type Change = SecretsChange;
    type Codec = SecretCodec;

    fn restore_from_replay(state: &ReplayState) -> Result<Self::Store> {
        let mut latest_snapshot = state
            .checkpoint
            .as_ref()
            .and_then(|checkpoint| checkpoint.find_feature(FeatureType::Secrets))
            .map(|feature| SecretCodec::decode_snapshot(feature.version, &feature.payload))
            .transpose()?;

        let mut deltas = Vec::new();

        for record in state
            .records
            .iter()
            .filter(|r| r.header.feature_type == FeatureType::Secrets)
        {
            let change = SecretCodec::decode_change(record.header.version, &record.payload)?;

            match change {
                SecretsChange::Snapshot(snapshot) => {
                    latest_snapshot = Some(snapshot);
                    deltas.clear();
                }
                SecretsChange::Deltas(batch) => deltas.extend(batch),
            }
        }

        let snapshot = latest_snapshot.unwrap_or_else(|| SecretStore::new().snapshot());

        SecretStore::restore(snapshot, deltas).map_err(Into::into)
    }

    fn load(session: &mut VaultSession) -> Result<Self::Store> {
        let replay = replay_since_checkpoint(session)?;
        Self::restore_from_replay(&replay)
    }

    fn commit(session: &mut VaultSession, store: &mut Self::Store) -> Result<CommitOutcome> {
        let feature_type = FeatureType::Secrets;

        let Some(change) = store.pending_changes() else {
            return Ok(CommitOutcome::no_change(feature_type));
        };

        let encoded = SecretCodec::encode_change(change)?;

        let mut record = Record::new(feature_type, SecretCodec::wire_version(), encoded);
        append_record(session, &mut record)?;

        store.clear_deltas();

        Ok(CommitOutcome::persisted(feature_type))
    }

    fn create_checkpoint(store: &Self::Store) -> Result<CheckpointFeature> {
        let checkpoint_payload = SecretCodec::encode_snapshot(store.snapshot())?;

        Ok(CheckpointFeature {
            feature_type: FeatureType::Secrets,
            version: SecretCodec::wire_version(),
            payload: checkpoint_payload,
        })
    }

    fn referenced_blobs(_store: &Self::Store) -> Vec<BlobRef> {
        Vec::new()
    }

    fn rewrite_blob_refs(_store: &mut Self::Store, _remap: &HashMap<BlobRef, BlobRef>) -> Result {
        Ok(())
    }
}
