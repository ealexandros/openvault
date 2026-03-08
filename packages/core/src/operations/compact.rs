use std::collections::HashMap;
use std::fs::File;
use std::io;

use uuid::Uuid;

use crate::errors::{Error, Result};
use crate::features::filesystem::{FILESYSTEM_WIRE_VERSION, FilesystemCodec, FilesystemSnapshot};
use crate::features::shared::{BlobRef, FeatureCodec};
use crate::internal::fs::open_with_read_write;
use crate::internal::io_ext::SeekExt;
use crate::operations::replay::replay_since_checkpoint;
use crate::repositories::{FeatureRepository, FilesystemRepository};
use crate::vault::features::FeatureType;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::boot_header::BootHeader;
use crate::vault::versions::shared::checkpoint::{Checkpoint, CheckpointFeature};
use crate::vault::versions::shared::format::FormatContext;
use crate::vault::versions::shared::replay::ReplayState;

const COMPACT_TEMP_SUFFIX: &str = ".compact-tmp";

// @todo-now refactor this

pub fn compact_vault(session: &mut VaultSession) -> Result {
    let replay = replay_since_checkpoint(session)?;
    ensure_only_filesystem_feature(&replay)?;

    let filesystem = FilesystemRepository::load(session)?;
    let mut filesystem_snapshot = filesystem.snapshot();

    let format = session.format();

    let vault_path = session.file_path().clone();
    let temp_path = temp_compact_path(&vault_path);

    let mut temp_file = File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&temp_path)?;

    let write_result = session.with_format_context(|source_file, context| {
        let boot_header = BootHeader::read_from(source_file)?;
        boot_header.write_to(&mut temp_file)?;
        format.init_layout(&mut temp_file, context)?;

        relocate_blobs(
            format,
            source_file,
            &mut temp_file,
            context,
            &mut filesystem_snapshot,
        )?;

        let mut checkpoint = Checkpoint::new(vec![CheckpointFeature {
            feature_type: FeatureType::Filesystem,
            version: FILESYSTEM_WIRE_VERSION,
            payload: FilesystemCodec::encode_snapshot(filesystem_snapshot)?,
        }]);

        format.write_checkpoint(&mut temp_file, &mut checkpoint, context)?;
        temp_file.sync_all()?;

        Ok(())
    });

    if let Err(error) = write_result {
        drop(temp_file);
        let _ = std::fs::remove_file(&temp_path);
        return Err(error);
    }

    drop(temp_file);

    let rewrite_result = rewrite_current_vault(session, &temp_path);
    let cleanup_result = std::fs::remove_file(&temp_path);

    rewrite_result?;
    cleanup_result.map_err(Error::from)?;

    Ok(())
}

fn ensure_only_filesystem_feature(replay: &ReplayState) -> Result {
    let has_non_filesystem_record = replay
        .records
        .iter()
        .any(|record| record.header.feature_type != FeatureType::Filesystem);

    if has_non_filesystem_record {
        return Err(Error::FeatureCodec(
            "Compaction only supports filesystem records".to_string(),
        ));
    }

    let has_non_filesystem_checkpoint_feature =
        replay.checkpoint.as_ref().is_some_and(|checkpoint| {
            checkpoint
                .features
                .iter()
                .any(|feature| feature.feature_type != FeatureType::Filesystem)
        });

    if has_non_filesystem_checkpoint_feature {
        return Err(Error::FeatureCodec(
            "Compaction only supports filesystem checkpoints".to_string(),
        ));
    }

    Ok(())
}

fn relocate_blobs(
    format: crate::vault::versions::factory::FormatRef,
    source_file: &mut File,
    target_file: &mut File,
    context: &FormatContext<'_>,
    snapshot: &mut FilesystemSnapshot,
) -> Result {
    let mut blob_refs: Vec<BlobRef> = snapshot.files.values().map(|f| f.blob.clone()).collect();
    blob_refs.sort_by(|a, b| {
        a.manifest_offset
            .cmp(&b.manifest_offset)
            .then_with(|| a.id.as_u128().cmp(&b.id.as_u128()))
    });
    blob_refs.dedup();

    let mut rewritten_refs = HashMap::with_capacity(blob_refs.len());

    for blob_ref in blob_refs {
        let blob_bytes = format.read_blob(source_file, &blob_ref, context)?;
        let mut cursor = io::Cursor::new(blob_bytes);
        let new_blob_ref = format.write_blob(target_file, &mut cursor, context)?;

        rewritten_refs.insert(blob_ref, new_blob_ref);
    }

    for file in snapshot.files.values_mut() {
        let Some(new_ref) = rewritten_refs.get(&file.blob) else {
            return Err(Error::InvalidVaultFormat);
        };

        file.blob = new_ref.clone();
    }

    Ok(())
}

fn rewrite_current_vault(session: &mut VaultSession, temp_path: &std::path::Path) -> Result {
    let mut compacted = open_with_read_write(temp_path)?;
    compacted.seek_to_start()?;

    let current = session.file_mut();
    current.set_len(0)?;
    current.seek_to_start()?;

    io::copy(&mut compacted, current)?;
    current.sync_all()?;

    Ok(())
}

fn temp_compact_path(vault_path: &std::path::Path) -> std::path::PathBuf {
    let filename = vault_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("vault");

    let compact_name = format!("{filename}{COMPACT_TEMP_SUFFIX}-{}", Uuid::new_v4());
    vault_path.with_file_name(compact_name)
}
