use crate::vault::features::FeatureType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Checkpoint {
    pub features: Vec<Feature>,
    pub last_delta_sequence: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub feature_id: FeatureType,
    pub version: u16,
    pub payload: Vec<u8>,
}
