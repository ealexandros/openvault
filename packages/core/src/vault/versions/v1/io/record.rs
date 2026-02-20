use crate::errors::{Error, Result};
use crate::internal::io_ext::{Reader, Rw};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::record::{RecordHeader, RecordWire};
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};
use crate::vault::versions::v1::io::subheader::{read_subheader, write_subheader};
use crate::vault::versions::v1::mapper::{decode_record, encode_record};
use std::io::SeekFrom;

pub fn append_record(
    rw: &mut Rw,
    record: &RecordHeader,
    payload: &[u8],
    keyring: &Keyring,
) -> Result<u64> {
    let mut subheader = read_subheader(rw, keyring)?;
    rw.seek(SeekFrom::End(0))?;

    let mut record = record.clone();

    record.sequence = subheader.last_sequence + 1;
    record.prev_record_offset = subheader.tail_record_offset;

    let record_wire = encode_record(&record, payload)?;
    let record_offset = seal_frame(rw, AadDomain::Record, &record_wire, keyring)?;

    subheader.tail_record_offset = record_offset;
    subheader.last_sequence += 1;
    write_subheader(rw, &subheader, keyring)?;

    Ok(record_offset)
}

pub fn read_record(reader: &mut Reader, offset: u64, keyring: &Keyring) -> Result<RecordWire> {
    reader.seek(SeekFrom::Start(offset))?;
    let record_wire = open_frame(reader, AadDomain::Record, keyring)?;
    let (record, payload) = decode_record(&record_wire)?;
    Ok(RecordWire::new(record, payload))
}

pub fn read_replay_records(
    reader: &mut Reader,
    start_offset: u64,
    keyring: &Keyring,
) -> Result<Vec<(u64, RecordWire)>> {
    let mut current_offset = start_offset;
    let mut last_sequence: Option<u64> = None;
    let mut records: Vec<(u64, RecordWire)> = Vec::new();

    while current_offset != 0 {
        let offset = current_offset;

        let record_wire = read_record(reader, offset, keyring)?;
        let prev_record_offset = record_wire.header.prev_record_offset;

        if last_sequence.is_some_and(|s| record_wire.header.sequence >= s) {
            return Err(Error::InvalidVaultFormat);
        }
        last_sequence = Some(record_wire.header.sequence);

        if prev_record_offset != 0 && prev_record_offset >= offset {
            return Err(Error::InvalidVaultFormat);
        }

        current_offset = prev_record_offset;
        records.push((offset, record_wire));
    }

    records.reverse();

    Ok(records)
}
