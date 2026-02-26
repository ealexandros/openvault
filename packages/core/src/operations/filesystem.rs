use crate::errors::Result;
use crate::features::filesystem::{FilesystemChange, FilesystemCodec, FilesystemStore};
use crate::features::shared::feature_trait::FeatureCodec;
use crate::operations::replay::replay_since_checkpoint;
use crate::vault::features::FeatureType;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::record::Record;

pub struct FilesystemOps;

impl FilesystemOps {
    pub fn apply(session: &mut VaultSession, change: FilesystemChange) -> Result<u64> {
        let codec = FilesystemCodec;

        let encoded = codec.encode_change(change)?;
        let mut record = Record::new(FeatureType::Filesystem, codec.wire_version(), encoded);

        let format = session.format();

        session
            .with_format_context(|file, context| format.append_record(file, &mut record, context))
    }

    pub fn load(session: &mut VaultSession) -> Result<FilesystemStore> {
        let codec = FilesystemCodec;
        let replay = replay_since_checkpoint(session)?;

        let mut latest_snapshot = None;
        let mut deltas = Vec::new();

        for record in replay.records {
            if record.header.feature_type != FeatureType::Filesystem {
                continue;
            }

            let change = codec.decode_change(record.header.version, &record.payload)?;

            match change {
                FilesystemChange::Snapshot(snapshot) => {
                    latest_snapshot = Some(snapshot);
                    deltas.clear();
                }
                FilesystemChange::Deltas(batch) => deltas.extend(batch),
            }
        }

        let snapshot = latest_snapshot.unwrap_or_else(|| FilesystemStore::new().snapshot());

        FilesystemStore::restore(snapshot, deltas).map_err(Into::into)
    }

    pub fn commit(session: &mut VaultSession, store: &mut FilesystemStore) -> Result<bool> {
        let Some(change) = store.pending_changes() else {
            return Ok(false);
        };

        FilesystemOps::apply(session, change)?;

        store.clear_deltas();

        Ok(true)
    }
}
