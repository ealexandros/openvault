use serde::{Deserialize, Serialize};
use std::mem::size_of;

use crate::vault::features::FeatureType;

pub const RECORD_SIZE: usize = size_of::<Record>();

// @todo-now remove clone

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Record {
    pub feature_id: FeatureType,
    pub version: u16,
    pub sequence: u64,
    pub prev_record_offset: u64,
    pub payload: Vec<u8>,
}
