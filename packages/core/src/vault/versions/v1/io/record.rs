use crate::errors::{Error, Result};
use crate::internal::io_ext::{ReadWriter, Reader, SeekExt};
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::traits::FormatContext;
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};
use crate::vault::versions::v1::io::subheader::{read_subheader, write_subheader};
use crate::vault::versions::v1::mapper::{decode_record, encode_record};

pub fn append_record(
    rw: &mut ReadWriter,
    record: &mut Record,
    context: &FormatContext,
) -> Result<u64> {
    let mut subheader = read_subheader(rw, context)?;

    rw.seek_to_end()?;

    record.header.sequence = subheader.last_sequence + 1;
    record.header.prev_record_offset = subheader.tail_record_offset;

    let record_bytes = encode_record(&record)?;
    let record_offset = seal_frame(rw, AadDomain::Record, &record_bytes, context)?;

    subheader.tail_record_offset = record_offset;
    subheader.last_sequence += 1;

    write_subheader(rw, &subheader, context)?;

    Ok(record_offset)
}

pub fn read_record(reader: &mut Reader, offset: u64, context: &FormatContext) -> Result<Record> {
    reader.seek_from_start(offset)?;
    let record_bytes = open_frame(reader, AadDomain::Record, context)?;
    decode_record(&record_bytes)
}

pub fn read_replay_records(
    reader: &mut Reader,
    start_offset: u64,
    stop_offset: u64,
    context: &FormatContext,
) -> Result<Vec<(u64, Record)>> {
    let mut current_offset = start_offset;
    let mut last_sequence = None;
    let mut records = Vec::new();

    while current_offset != 0 && current_offset >= stop_offset {
        let offset = current_offset;

        let record_wire = read_record(reader, offset, context)?;
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
