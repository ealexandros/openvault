use std::collections::HashMap;

use crate::errors::Result;
use crate::features::FeatureType;
use crate::features::messages::{MessagesChange, MessagesCodec, MessagesStore};
use crate::features::shared::{BlobRef, FeatureCodec};
use crate::operations::history::append_record;
use crate::repositories::{CommitOutcome, FeatureRepository};
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::checkpoint::CheckpointFeature;
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::replay::ReplayState;

pub struct MessagesRepository;

impl FeatureRepository for MessagesRepository {
    type Store = MessagesStore;
    type Change = MessagesChange;
    type Codec = MessagesCodec;

    fn restore_from_replay(state: &ReplayState) -> Result<Self::Store> {
        let mut latest_snapshot = state
            .checkpoint
            .as_ref()
            .and_then(|checkpoint| checkpoint.find_feature(FeatureType::Messages))
            .map(|feature| MessagesCodec::decode_snapshot(feature.version, &feature.payload))
            .transpose()?;

        let mut deltas = Vec::new();

        for record in state
            .records
            .iter()
            .filter(|r| r.header.feature_type == FeatureType::Messages)
        {
            let change = MessagesCodec::decode_change(record.header.version, &record.payload)?;

            match change {
                MessagesChange::Snapshot(snapshot) => {
                    latest_snapshot = Some(snapshot);
                    deltas.clear();
                }
                MessagesChange::Deltas(batch) => deltas.extend(batch),
            }
        }

        let snapshot = latest_snapshot.unwrap_or_else(|| MessagesStore::new().snapshot());

        MessagesStore::restore(snapshot, deltas).map_err(Into::into)
    }

    fn commit(session: &mut VaultSession, store: &mut Self::Store) -> Result<CommitOutcome> {
        let feature_type = FeatureType::Messages;

        let Some(change) = store.pending_changes() else {
            return Ok(CommitOutcome::no_change(feature_type));
        };

        let encoded = MessagesCodec::encode_change(change)?;

        let mut record = Record::new(feature_type, MessagesCodec::wire_version(), encoded);
        append_record(session, &mut record)?;

        store.clear_deltas();

        Ok(CommitOutcome::persisted(feature_type))
    }

    fn create_checkpoint(store: &Self::Store) -> Result<CheckpointFeature> {
        let checkpoint_payload = MessagesCodec::encode_snapshot(store.snapshot())?;

        Ok(CheckpointFeature {
            feature_type: FeatureType::Messages,
            version: MessagesCodec::wire_version(),
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
