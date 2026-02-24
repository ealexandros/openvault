use crate::vault::versions::shared::checkpoint::Checkpoint;
use crate::vault::versions::shared::record::RecordHeader;
use crate::vault::versions::shared::subheader::Subheader;

#[derive(Debug)]
pub struct ReplayRecord {
    pub offset: u64,
    pub header: RecordHeader,
    pub payload: Vec<u8>,
}

impl ReplayRecord {
    pub fn new(offset: u64, header: RecordHeader, payload: Vec<u8>) -> Self {
        Self {
            offset,
            header,
            payload,
        }
    }
}

#[derive(Debug)]
pub struct ReplayState {
    pub subheader: Subheader,
    pub checkpoint: Option<Checkpoint>,
    pub records: Vec<ReplayRecord>,
}

impl ReplayState {
    pub fn new(
        subheader: Subheader,
        checkpoint: Option<Checkpoint>,
        records: Vec<ReplayRecord>,
    ) -> Self {
        Self {
            subheader,
            checkpoint,
            records,
        }
    }
}
