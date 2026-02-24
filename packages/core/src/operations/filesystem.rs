use crate::errors::{Error, Result};
use crate::features::feature_trait::{FeatureCodec, RecordKind};
use crate::features::filesystem::{
    FILESYSTEM_FEATURE_ID, FilesystemChange, FilesystemCodec, FilesystemStore,
};
use crate::operations::replay::replay_since_checkpoint;
use crate::vault::features::FeatureType;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::record::RecordHeader;

pub fn apply_filesystem_change(
    session: &mut VaultSession,
    change: FilesystemChange,
) -> Result<u64> {
    let codec = FilesystemCodec;
    let encoded = codec.encode_change(change).map_err(Error::FeatureCodec)?;

    if encoded.feature_id != FILESYSTEM_FEATURE_ID {
        return Err(Error::InvalidVaultFormat);
    }

    let payload = pack_feature_payload(encoded.kind, &encoded.payload);
    let header = RecordHeader::new(FeatureType::Filesystem, encoded.version);
    let format = session.format();

    session
        .with_format_context(|file, context| format.append_record(file, &header, &payload, context))
}

pub fn load_filesystem_store(session: &mut VaultSession) -> Result<FilesystemStore> {
    let codec = FilesystemCodec;
    let replay = replay_since_checkpoint(session)?;

    let mut latest_snapshot = None;
    let mut deltas = Vec::new();

    for record in replay.records {
        if record.header.feature_id != FeatureType::Filesystem {
            continue;
        }

        let (kind, payload) = unpack_feature_payload(&record.payload)?;
        let change = codec
            .decode_change(record.header.version, kind, payload)
            .map_err(Error::FeatureCodec)?;

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

pub fn commit_filesystem_store(
    session: &mut VaultSession,
    store: &mut FilesystemStore,
) -> Result<bool> {
    let Some(change) = store.pending_changes() else {
        return Ok(false);
    };

    apply_filesystem_change(session, change)?;
    store.reset_sync_state();

    Ok(true)
}

fn pack_feature_payload(kind: RecordKind, payload: &[u8]) -> Vec<u8> {
    let mut framed = Vec::with_capacity(payload.len() + 1);
    framed.push(encode_record_kind(kind));
    framed.extend_from_slice(payload);
    framed
}

fn unpack_feature_payload(payload: &[u8]) -> Result<(RecordKind, &[u8])> {
    let Some((&kind, body)) = payload.split_first() else {
        return Err(Error::InvalidVaultFormat);
    };

    Ok((decode_record_kind(kind)?, body))
}

fn encode_record_kind(kind: RecordKind) -> u8 {
    match kind {
        RecordKind::Snapshot => 1,
        RecordKind::Delta => 2,
    }
}

fn decode_record_kind(value: u8) -> Result<RecordKind> {
    match value {
        1 => Ok(RecordKind::Snapshot),
        2 => Ok(RecordKind::Delta),
        _ => Err(Error::InvalidVaultFormat),
    }
}
