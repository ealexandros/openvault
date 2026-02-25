use std::fs::File;
use std::path::Path;
use uuid::Uuid;

use crate::errors::{Error, Result};
use crate::features::filesystem::{FilesystemChange, FilesystemCodec, FilesystemStore};
use crate::features::shared::feature_trait::FeatureCodec;
use crate::operations::blob;
use crate::operations::replay::replay_since_checkpoint;
use crate::vault::features::FeatureType;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::record::Record;

pub fn apply_filesystem_change(
    session: &mut VaultSession,
    change: FilesystemChange,
) -> Result<u64> {
    let codec = FilesystemCodec;

    let encoded = codec.encode_change(change)?;
    let mut record = Record::new(FeatureType::Filesystem, codec.wire_version(), encoded);

    let format = session.format();

    session.with_format_context(|file, context| format.append_record(file, &mut record, context))
}

pub fn load_filesystem_store(session: &mut VaultSession) -> Result<FilesystemStore> {
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

// @todo-now remove this from here..

pub fn add_file(
    session: &mut VaultSession,
    store: &mut FilesystemStore,
    parent_id: Uuid,
    source_path: &Path,
) -> Result<Uuid> {
    let name = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or(Error::InvalidPath)?
        .to_string();

    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_string();

    let mut file = File::open(source_path)?;
    let blob_ref = blob::put_blob(session, &mut file)?;

    let file_id = store.add_file(parent_id, name, extension, blob_ref)?;

    Ok(file_id)
}

pub fn commit_filesystem_store(
    session: &mut VaultSession,
    store: &mut FilesystemStore,
) -> Result<bool> {
    let Some(change) = store.pending_changes() else {
        return Ok(false);
    };

    apply_filesystem_change(session, change)?;
    store.clear_deltas();

    Ok(true)
}
