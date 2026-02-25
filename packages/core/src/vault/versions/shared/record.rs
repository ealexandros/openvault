use serde::{Deserialize, Serialize};

use crate::vault::features::FeatureType;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecordHeader {
    pub feature_type: FeatureType,
    pub version: u16,
    pub sequence: u64,
    pub prev_record_offset: u64,
}

impl RecordHeader {
    pub const SIZE: usize = 20;

    pub fn new(feature_type: FeatureType, version: u16) -> Self {
        Self {
            feature_type,
            version,
            sequence: 0,
            prev_record_offset: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub header: RecordHeader,
    pub payload: Vec<u8>,
}

impl Record {
    pub fn new(feature_type: FeatureType, version: u16, payload: Vec<u8>) -> Self {
        Self {
            header: RecordHeader::new(feature_type, version),
            payload,
        }
    }
}
