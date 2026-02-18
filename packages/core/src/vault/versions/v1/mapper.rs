use serde::{Deserialize, Serialize};

use crate::errors::{Error, Result};
use crate::vault::versions::shared::record::Record;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RecordWire {
    record: Record,
    payload: Vec<u8>,
}

pub fn encode_record(record: &Record, data: &[u8]) -> Result<Vec<u8>> {
    if record.payload_size as usize != data.len() {
        return Err(Error::InvalidVaultFormat);
    }

    let wire = RecordWire {
        record: record.clone(),
        payload: data.to_vec(),
    };

    postcard::to_allocvec(&wire).map_err(|_| Error::InvalidVaultFormat)
}

pub fn decode_record(bytes: &[u8]) -> Result<(Record, Vec<u8>)> {
    let wire: RecordWire = postcard::from_bytes(bytes).map_err(|_| Error::InvalidVaultFormat)?;

    if wire.record.payload_size as usize != wire.payload.len() {
        return Err(Error::InvalidVaultFormat);
    }

    Ok((wire.record, wire.payload))
}
