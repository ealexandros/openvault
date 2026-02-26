pub trait FeatureCodec {
    type Error;
    type DomainChange;

    fn feature_id(&self) -> &'static str;

    fn wire_version(&self) -> u16;

    fn encode_change(&self, change: Self::DomainChange) -> Result<Vec<u8>, Self::Error>;

    fn decode_change(
        &self,
        wire_version: u16,
        payload: &[u8],
    ) -> Result<Self::DomainChange, Self::Error>;
}
