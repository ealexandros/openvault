use crate::errors::Result;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::record::RecordHeader;
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};
use crate::vault::versions::v1::io::subheader::{read_subheader_from_rw, write_subheader};
use crate::vault::versions::v1::mapper::{decode_record, encode_record};
use std::io::SeekFrom;

pub fn append_record(
    writer: &mut dyn WriteSeek,
    record: &RecordHeader,
    payload: &[u8],
    keyring: &Keyring,
) -> Result<u64> {
    let mut subheader = read_subheader_from_rw(writer, keyring)?;
    writer.seek(SeekFrom::End(0))?;

    let mut record = record.clone();

    record.sequence = subheader.last_sequence + 1;
    record.prev_record_offset = subheader.tail_record_offset;

    let record_wire = encode_record(&record, payload)?;
    let record_offset = seal_frame(writer, AadDomain::Record, &record_wire, keyring)?;

    subheader.tail_record_offset = record_offset;
    subheader.last_sequence += 1;
    write_subheader(writer, &subheader, keyring)?;

    Ok(record_offset)
}

pub fn read_record(
    reader: &mut dyn ReadSeek,
    offset: u64,
    keyring: &Keyring,
) -> Result<(RecordHeader, Vec<u8>)> {
    reader.seek(SeekFrom::Start(offset))?;
    let record_wire = open_frame(reader, AadDomain::Record, keyring)?;
    let (record, payload) = decode_record(&record_wire)?;
    Ok((record, payload))
}

pub struct RecordIterator<'a> {
    reader: &'a mut dyn ReadSeek,
    current_offset: u64,
    keyring: &'a Keyring,
}

impl<'a> RecordIterator<'a> {
    pub fn new(reader: &'a mut dyn ReadSeek, start_offset: u64, keyring: &'a Keyring) -> Self {
        Self {
            reader,
            current_offset: start_offset,
            keyring,
        }
    }

    fn next_internal(&mut self) -> Result<Option<(u64, RecordHeader, Vec<u8>)>> {
        if self.current_offset == 0 {
            return Ok(None);
        }

        let offset = self.current_offset;
        let (record, payload) = read_record(self.reader, offset, self.keyring)?;

        // @todo-soon rethink unwrap_or

        self.current_offset = record.prev_record_offset;
        Ok(Some((offset, record, payload)))
    }
}

impl<'a> Iterator for RecordIterator<'a> {
    type Item = Result<(u64, RecordHeader, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_internal().transpose()
    }
}
