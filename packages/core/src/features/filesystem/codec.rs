use crate::features::shared::feature_trait::{EncodedFeatureRecord, FeatureCodec, RecordKind};

use super::error::FilesystemError;
use super::records::{FILESYSTEM_WIRE_VERSION_V1, FilesystemChange};

pub const FILESYSTEM_FEATURE_ID: &str = "filesystem";

#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemCodec;

impl FeatureCodec for FilesystemCodec {
    type DomainChange = FilesystemChange;

    fn feature_id(&self) -> &'static str {
        FILESYSTEM_FEATURE_ID
    }

    fn current_wire_version(&self) -> u16 {
        FILESYSTEM_WIRE_VERSION_V1
    }

    fn encode_change(
        &self,
        change: Self::DomainChange,
    ) -> std::result::Result<EncodedFeatureRecord, String> {
        let kind = expected_kind(&change);
        let payload = postcard::to_allocvec(&change)
            .map_err(|e| FilesystemError::InvalidPayload(e.to_string()).to_string())?;

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
        if wire_version != FILESYSTEM_WIRE_VERSION_V1 {
            return Err(FilesystemError::UnsupportedWireVersion(wire_version).to_string());
        }

        let change: FilesystemChange = postcard::from_bytes(payload)
            .map_err(|e| FilesystemError::InvalidPayload(e.to_string()).to_string())?;

        let expected = expected_kind(&change);
        if kind != expected {
            return Err(FilesystemError::InvalidRecordKind {
                expected,
                actual: kind,
            }
            .to_string());
        }

        Ok(change)
    }
}

fn expected_kind(change: &FilesystemChange) -> RecordKind {
    match change {
        FilesystemChange::Snapshot(_) => RecordKind::Snapshot,
        FilesystemChange::Deltas(_) => RecordKind::Delta,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid::Uuid;

    use crate::features::shared::feature_trait::FeatureCodec;

    use super::*;
    use crate::features::filesystem::{
        FileMetadata, FilesystemDelta, FilesystemSnapshot, FolderMetadata, ROOT_FOLDER_ID,
    };
    use crate::features::shared::blob_ref::BlobRef;

    #[test]
    fn codec_roundtrip_snapshot_payload() {
        let codec = FilesystemCodec;

        let mut folders = HashMap::new();

        let docs = FolderMetadata::new(Some(ROOT_FOLDER_ID), "docs");
        let docs_id = docs.id;
        folders.insert(docs_id, docs);

        let mut files = HashMap::new();
        let mut file = FileMetadata::new(docs_id, "report.txt");

        file.blob = Some(BlobRef {
            id: Uuid::nil(),
            size_bytes: 256,
            manifest_offset: 0,
        });
        files.insert(file.id, file);

        let change = FilesystemChange::Snapshot(FilesystemSnapshot::new(folders, files));
        let encoded = codec.encode_change(change.clone()).expect("encode");
        let decoded = codec
            .decode_change(encoded.version, encoded.kind, &encoded.payload)
            .expect("decode");

        assert_eq!(encoded.feature_id, FILESYSTEM_FEATURE_ID);
        assert_eq!(encoded.kind, RecordKind::Snapshot);
        assert_eq!(decoded, change);
    }

    #[test]
    fn codec_roundtrip_delta_payload() {
        let codec = FilesystemCodec;

        let file_id = Uuid::new_v4();
        let change = FilesystemChange::Deltas(vec![FilesystemDelta::FileDeleted { id: file_id }]);
        let encoded = codec.encode_change(change.clone()).expect("encode");
        let decoded = codec
            .decode_change(encoded.version, encoded.kind, &encoded.payload)
            .expect("decode");

        assert_eq!(encoded.kind, RecordKind::Delta);
        assert_eq!(decoded, change);
    }

    #[test]
    fn codec_rejects_unsupported_wire_version() {
        let codec = FilesystemCodec;
        let change = FilesystemChange::Deltas(vec![]);
        let encoded = codec.encode_change(change).expect("encode");

        let err = codec
            .decode_change(
                FILESYSTEM_WIRE_VERSION_V1 + 1,
                encoded.kind,
                &encoded.payload,
            )
            .expect_err("version mismatch");

        assert!(err.contains("Unsupported filesystem wire version"));
    }

    #[test]
    fn codec_rejects_record_kind_mismatch() {
        let codec = FilesystemCodec;
        let change = FilesystemChange::Snapshot(FilesystemSnapshot::default());
        let encoded = codec.encode_change(change).expect("encode");

        let err = codec
            .decode_change(encoded.version, RecordKind::Delta, &encoded.payload)
            .expect_err("kind mismatch");

        assert!(err.contains("Invalid feature record kind"));
    }
}
