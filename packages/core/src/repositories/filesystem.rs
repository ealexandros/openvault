use std::collections::HashMap;

use crate::errors::{Error, Result};
use crate::features::FeatureType;
use crate::features::filesystem::{FilesystemChange, FilesystemCodec, FilesystemStore};
use crate::features::shared::{BlobRef, FeatureCodec};
use crate::operations::history::append_record;
use crate::operations::replay::replay_since_checkpoint;
use crate::repositories::{CommitOutcome, FeatureRepository};
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::checkpoint::CheckpointFeature;
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::replay::ReplayState;

pub struct FilesystemRepository;

impl FeatureRepository for FilesystemRepository {
    type Store = FilesystemStore;
    type Change = FilesystemChange;
    type Codec = FilesystemCodec;

    fn restore_from_replay(state: &ReplayState) -> Result<Self::Store> {
        let mut latest_snapshot = state
            .checkpoint
            .as_ref()
            .and_then(|checkpoint| checkpoint.find_feature(FeatureType::Filesystem))
            .map(|feature| FilesystemCodec::decode_snapshot(feature.version, &feature.payload))
            .transpose()?;

        let mut deltas = Vec::new();

        for record in state
            .records
            .iter()
            .filter(|r| r.header.feature_type == FeatureType::Filesystem)
        {
            let change = FilesystemCodec::decode_change(record.header.version, &record.payload)?;

            match change {
                FilesystemChange::Snapshot(s) => {
                    latest_snapshot = Some(s);
                    deltas.clear();
                }
                FilesystemChange::Deltas(batch) => deltas.extend(batch),
            }
        }

        let snapshot = latest_snapshot.unwrap_or_else(|| FilesystemStore::new().snapshot());

        FilesystemStore::restore(snapshot, deltas).map_err(Into::into)
    }

    fn load(session: &mut VaultSession) -> Result<Self::Store> {
        let replay = replay_since_checkpoint(session)?;
        Self::restore_from_replay(&replay)
    }

    fn commit(session: &mut VaultSession, store: &mut Self::Store) -> Result<CommitOutcome> {
        let feature_type = FeatureType::Filesystem;

        let Some(change) = store.pending_changes() else {
            return Ok(CommitOutcome::no_change(feature_type));
        };

        let encoded = FilesystemCodec::encode_change(change)?;

        let mut record = Record::new(feature_type, FilesystemCodec::wire_version(), encoded);
        append_record(session, &mut record)?;

        store.clear_deltas();

        Ok(CommitOutcome::persisted(feature_type))
    }

    fn create_checkpoint(store: &Self::Store) -> Result<CheckpointFeature> {
        let checkpoint_payload = FilesystemCodec::encode_snapshot(store.snapshot())?;

        Ok(CheckpointFeature {
            feature_type: FeatureType::Filesystem,
            version: FilesystemCodec::wire_version(),
            payload: checkpoint_payload,
        })
    }

    fn referenced_blobs(store: &Self::Store) -> Vec<BlobRef> {
        store
            .snapshot()
            .files
            .values()
            .map(|file| file.blob.clone())
            .collect()
    }

    fn rewrite_blob_refs(store: &mut Self::Store, remap: &HashMap<BlobRef, BlobRef>) -> Result {
        for file in store.files.values_mut() {
            file.blob = remap
                .get(&file.blob)
                .cloned()
                .ok_or(Error::InvalidVaultFormat)?;
        }

        Ok(())
    }
}
