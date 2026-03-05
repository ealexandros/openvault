pub trait FeatureCodec {
    type Error;
    type DomainChange;
    type DomainSnapshot: Into<Self::DomainChange> + TryFrom<Self::DomainChange, Error = Self::Error>;

    fn wire_version(&self) -> u16;

    fn encode_change(&self, change: Self::DomainChange) -> Result<Vec<u8>, Self::Error>;

    fn decode_change(
        &self,
        wire_version: u16,
        payload: &[u8],
    ) -> Result<Self::DomainChange, Self::Error>;

    fn encode_snapshot(&self, snapshot: Self::DomainSnapshot) -> Result<Vec<u8>, Self::Error> {
        self.encode_change(snapshot.into())
    }

    fn decode_snapshot(
        &self,
        wire_version: u16,
        payload: &[u8],
    ) -> Result<Self::DomainSnapshot, Self::Error> {
        self.decode_change(wire_version, payload)?.try_into()
    }
}
