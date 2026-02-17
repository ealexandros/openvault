use serde::{Deserialize, Serialize};

/// A tag byte written before each record to identify its type.
#[repr(u8)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RecordType {
    Delta = 1,
    Snapshot = 2,
}

/// Each record appended to the vault file has this layout:
/// [record_type: u8][prev_offset: u64][payload_size: u32][payload: Vec<u8>]
///
/// The payload is already encrypted by the time it reaches here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub record_type: RecordType,
    pub prev_offset: u64,
    pub payload_size: u32,
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub enum DeltaPayload {
//     FileAdded(FileMeta),
//     FileDeleted { id: u32 },
//     FolderAdded(FolderMeta),
//     FolderDeleted { id: u32 },
//     NoteAdded(NoteMeta),
//     NoteUpdated(NoteMeta),
//     NoteDeleted { id: u32 },
//     SecretChunk(Vec<u8>), // opaque blob from SecretManager
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SnapshotPayload {
//     pub filesystem: FileSystemMeta,
//     pub notes: Vec<NoteMeta>,
//     pub secret_chunks: Vec<Vec<u8>>,
// }
