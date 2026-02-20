use crate::vault::features::FeatureType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Checkpoint {
    pub features: Vec<FeatureHeader>,
    pub last_delta_sequence: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureHeader {
    pub feature_id: FeatureType,
    pub version: u16,
    pub snapshot_offset: u64,
}
