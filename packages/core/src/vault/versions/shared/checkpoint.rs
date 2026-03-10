use serde::{Deserialize, Serialize};

use crate::features::FeatureType;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Checkpoint {
    pub features: Vec<CheckpointFeature>,
    pub sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointFeature {
    pub feature_type: FeatureType,
    pub version: u16,
    pub payload: Vec<u8>,
}

impl Checkpoint {
    pub fn new(features: Vec<CheckpointFeature>) -> Self {
        Self {
            features,
            sequence: 0,
        }
    }

    pub fn find_feature(&self, feature_type: FeatureType) -> Option<CheckpointFeature> {
        self.features
            .iter()
            .find(|f| f.feature_type == feature_type)
            .cloned()
    }
}
