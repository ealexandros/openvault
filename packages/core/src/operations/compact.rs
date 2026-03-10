use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::errors::Result;
use crate::features::FeatureType;
use crate::features::shared::BlobRef;
use crate::internal::fs::open_with_read_write;
use crate::internal::io_ext::SeekExt;
use crate::operations::replay::replay_since_checkpoint;
use crate::repositories::FeatureRepository;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::boot_header::BootHeader;
use crate::vault::versions::shared::checkpoint::{Checkpoint, CheckpointFeature};
use crate::vault::versions::shared::format::FormatContext;
use crate::vault::versions::shared::replay::ReplayState;

const COMPACT_TEMP_SUFFIX: &str = ".compact-tmp";

pub struct CompactionBundle {
    pub feature_type: FeatureType,
    pub blob_refs: Vec<BlobRef>,
    remap_fn: Box<dyn FnOnce(&HashMap<BlobRef, BlobRef>) -> Result<CheckpointFeature>>,
}

pub fn compact_vault(session: &mut VaultSession) -> Result {
    let replay = replay_since_checkpoint(session)?;
    let feature_types = collect_present_features(&replay);

    let bundles: Vec<_> = feature_types
        .iter()
        .map(|ft| ft.build_bundle(session))
        .collect::<Result<_>>()?;

    let blob_refs = collect_unique_blob_refs_from_bundles(&bundles);

    let format = session.format();
    let vault_path = session.file_path();
    let temp_path = temp_compact_path(vault_path);

    let mut temp_file = File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&temp_path)?;

    let _cleanup = TempFileGuard(temp_path.clone());

    let remap = session.with_format_context(|source_file, context| {
        rewrite_vault(format, source_file, &mut temp_file, context, &blob_refs)
    })?;

    let checkpoint_features: Vec<_> = bundles
        .into_iter()
        .map(|bundle| (bundle.remap_fn)(&remap))
        .collect::<Result<_>>()?;

    session.with_format_context(|_, context| {
        let mut checkpoint = Checkpoint::new(checkpoint_features);
        format.write_checkpoint(&mut temp_file, &mut checkpoint, context)
    })?;

    temp_file.sync_all()?;
    rewrite_current_vault(session, &temp_path)?;

    Ok(())
}

pub fn build_bundle_for<R: FeatureRepository>(
    session: &mut VaultSession,
    feature_type: FeatureType,
) -> Result<CompactionBundle>
where
    R::Store: 'static,
{
    let mut store = R::load(session)?;
    let blob_refs = R::referenced_blobs(&store);

    let remap_fn = Box::new(move |remap: &HashMap<BlobRef, BlobRef>| {
        R::rewrite_blob_refs(&mut store, remap)?;
        R::create_checkpoint(&store)
    });

    Ok(CompactionBundle {
        feature_type,
        blob_refs,
        remap_fn,
    })
}

fn collect_present_features(replay: &ReplayState) -> Vec<FeatureType> {
    let mut set = HashSet::new();

    if let Some(checkpoint) = &replay.checkpoint {
        set.extend(checkpoint.features.iter().map(|f| f.feature_type));
    }

    set.extend(replay.records.iter().map(|r| r.header.feature_type));

    set.into_iter().collect()
}

fn collect_unique_blob_refs_from_bundles(bundles: &[CompactionBundle]) -> Vec<BlobRef> {
    let mut refs: Vec<_> = bundles
        .iter()
        .flat_map(|b| b.blob_refs.iter().cloned())
        .collect();

    refs.sort_by_key(|b| (b.manifest_offset, b.id.as_u128()));
    refs.dedup();

    refs
}

fn rewrite_vault(
    format: crate::vault::versions::factory::FormatRef,
    source: &mut File,
    target: &mut File,
    context: &FormatContext<'_>,
    blob_refs: &[BlobRef],
) -> Result<HashMap<BlobRef, BlobRef>> {
    let boot_header = BootHeader::read_from(source)?;
    boot_header.write_to(target)?;

    format.init_layout(target, context)?;

    let mut remap = HashMap::with_capacity(blob_refs.len());

    for blob in blob_refs {
        let bytes = format.read_blob(source, blob, context)?;
        let mut cursor = io::Cursor::new(bytes);

        let new_ref = format.write_blob(target, &mut cursor, context)?;
        remap.insert(blob.clone(), new_ref);
    }

    Ok(remap)
}

fn rewrite_current_vault(session: &mut VaultSession, temp_path: &Path) -> Result {
    let mut compacted = open_with_read_write(temp_path)?;
    compacted.seek_to_start()?;

    let current = session.file_mut();
    current.set_len(0)?;
    current.seek_to_start()?;

    io::copy(&mut compacted, current)?;
    current.sync_all()?;

    Ok(())
}

fn temp_compact_path(vault_path: &Path) -> PathBuf {
    let filename = vault_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("vault");

    vault_path.with_file_name(format!(
        "{filename}{COMPACT_TEMP_SUFFIX}-{}",
        Uuid::new_v4()
    ))
}

struct TempFileGuard(PathBuf);

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
