use crate::errors::{Error, Result};
use crate::vault::versions::shared::record::{RecordHeader, RecordWire};

pub fn encode_record(record: &RecordHeader, data: &[u8]) -> Result<Vec<u8>> {
    let wire = RecordWire {
        header: record.clone(),
        payload: data.to_vec(),
    };
    postcard::to_allocvec(&wire).map_err(|_| Error::InvalidVaultFormat)
}

pub fn decode_record(bytes: &[u8]) -> Result<(RecordHeader, Vec<u8>)> {
    let wire: RecordWire = postcard::from_bytes(bytes).map_err(|_| Error::InvalidVaultFormat)?;

    Ok((wire.header, wire.payload))
}
