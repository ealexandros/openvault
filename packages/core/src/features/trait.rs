#[derive(Debug, Clone, Copy)]
pub enum RecordKind {
    Snapshot,
    Delta,
}

#[derive(Debug, Clone)]
pub struct EncodedFeatureRecord {
    pub feature_id: &'static str,
    pub wire_version: u16,
    pub kind: RecordKind,
    pub payload: Vec<u8>,
}

pub trait FeatureCodec {
    type DomainChange;

    fn feature_id(&self) -> &'static str;
    fn current_wire_version(&self) -> u16;

    fn encode_change(&self, change: Self::DomainChange) -> Result<EncodedFeatureRecord, String>;

    fn decode_change(
        &self,
        wire_version: u16,
        kind: RecordKind,
        payload: &[u8],
    ) -> Result<Self::DomainChange, String>;
}
