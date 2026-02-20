use serde::{Deserialize, Serialize};
use std::mem::size_of;

use crate::vault::features::FeatureType;

pub const RECORD_SIZE: usize = size_of::<RecordHeader>();

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecordHeader {
    pub feature_id: FeatureType,
    pub version: u16,
    pub sequence: u64,
    pub prev_record_offset: u64,
}

impl RecordHeader {
    pub fn new(feature_id: FeatureType, version: u16) -> Self {
        Self {
            feature_id,
            version,
            sequence: 0,
            prev_record_offset: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecordWire {
    pub header: RecordHeader,
    pub payload: Vec<u8>,
}

impl RecordWire {
    pub fn new(header: RecordHeader, payload: Vec<u8>) -> Self {
        Self { header, payload }
    }
}
