use crate::vault::features::FeatureType;

pub struct Checkpoint {
    pub features: Vec<FeatureHeader>,
    pub last_delta_sequence: u64,
}

pub struct FeatureHeader {
    pub feature_id: FeatureType,
    pub version: u16,
    pub snapshot_offset: u64,
}
