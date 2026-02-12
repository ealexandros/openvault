use crate::vault::features::FeatureType;

pub struct Checkpoint {
    pub global_seq: u64,
    pub features: Vec<FeatureHeader>,
}

pub struct FeatureHeader {
    pub feature_id: FeatureType,
    pub payload_version: u16,
    pub snapshot_offset: u64,
    // pub last_delta_offset: u64,
    // pub last_seq: u64,
}
