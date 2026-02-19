use serde::{Deserialize, Serialize};
use std::mem::size_of;

use crate::vault::features::FeatureType;

pub const RECORD_SIZE: usize = size_of::<Record>();

#[repr(u8)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecordKind {
    Delta = 1,
    Blob = 2,
}

impl From<RecordKind> for u8 {
    fn from(kind: RecordKind) -> Self {
        kind as u8
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Record {
    pub kind: RecordKind,
    pub feature_id: FeatureType,
    pub payload_version: u16,
    pub sequence: u64,
    pub prev_offset: u64,
    // @todo-soon not used yet
    // pub payload_size: u32,
    pub key_epoch: u16,
}
