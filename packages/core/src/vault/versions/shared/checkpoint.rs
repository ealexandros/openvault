use serde::{Deserialize, Serialize};

use crate::vault::features::FeatureType;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Checkpoint {
    pub features: Vec<Feature>,
    pub sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub feature_type: FeatureType,
    pub version: u16,
    pub payload: Vec<u8>,
}

impl Checkpoint {
    pub fn find_feature(&self, feature_type: FeatureType) -> Option<Feature> {
        self.features
            .iter()
            .find(|f| f.feature_type == feature_type)
            .cloned()
    }
}
