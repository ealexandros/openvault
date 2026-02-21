use crate::errors::{Error, Result};
use crate::vault::versions::shared::checkpoint::Checkpoint;
use crate::vault::versions::shared::record::{RecordHeader, RecordWire};
use crate::vault::versions::v1::blob::BlobManifest;

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

pub fn encode_checkpoint(checkpoint: &Checkpoint) -> Result<Vec<u8>> {
    postcard::to_allocvec(checkpoint).map_err(|_| Error::InvalidVaultFormat)
}

pub fn decode_checkpoint(bytes: &[u8]) -> Result<Checkpoint> {
    postcard::from_bytes(bytes).map_err(|_| Error::InvalidVaultFormat)
}

pub fn encode_manifest(manifest: &BlobManifest) -> Result<Vec<u8>> {
    postcard::to_allocvec(manifest).map_err(|_| Error::InvalidVaultFormat)
}

pub fn decode_manifest(bytes: &[u8]) -> Result<BlobManifest> {
    postcard::from_bytes(bytes).map_err(|_| Error::InvalidVaultFormat)
}
