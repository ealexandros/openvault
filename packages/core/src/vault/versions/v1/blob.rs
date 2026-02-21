use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobChunkMeta {
    pub offset: u64,
    pub size: u32,
}

impl BlobChunkMeta {
    pub fn new(offset: u64, size: u32) -> Self {
        Self { offset, size }
    }

    pub fn size_bytes(&self) -> u64 {
        self.size as u64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobManifest {
    pub id: Uuid,
    pub version: u16,
    pub size_bytes: u64,
    pub chunk_size: u32,
    pub chunks: Vec<BlobChunkMeta>,
}
