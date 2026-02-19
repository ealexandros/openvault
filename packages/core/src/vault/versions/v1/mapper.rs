use crate::errors::{Error, Result};
use crate::vault::versions::shared::record::Record;

pub fn encode_record(record: &Record) -> Result<Vec<u8>> {
    postcard::to_allocvec(record).map_err(|_| Error::InvalidVaultFormat)
}

pub fn decode_record(bytes: &[u8]) -> Result<Record> {
    postcard::from_bytes(bytes).map_err(|_| Error::InvalidVaultFormat)
}
