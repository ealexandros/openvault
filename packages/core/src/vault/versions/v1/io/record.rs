use crate::errors::Result;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};
use crate::vault::versions::v1::io::subheader::{read_subheader_from_rw, write_subheader};
use crate::vault::versions::v1::mapper::{decode_record, encode_record};
use std::io::SeekFrom;

pub fn append_record(
    writer: &mut dyn WriteSeek,
    record: &Record,
    payload: &[u8],
    keyring: &Keyring,
) -> Result<u64> {
    let mut subheader = read_subheader_from_rw(writer, keyring)?;
    writer.seek(SeekFrom::End(0))?;

    let mut record = record.clone();

    record.sequence = subheader.last_sequence + 1;
    record.prev_record_offset = subheader.tail_record_offset;

    let record_wire = encode_record(&record)?;
    let record_offset = seal_frame(writer, AadDomain::Record, &record_wire, keyring)?;

    seal_frame(writer, AadDomain::Payload, payload, keyring)?;

    subheader.tail_record_offset = record_offset;
    subheader.last_sequence += 1;
    write_subheader(writer, &subheader, keyring)?;

    Ok(record_offset)
}

pub fn read_record(reader: &mut dyn ReadSeek, offset: u64, keyring: &Keyring) -> Result<Record> {
    reader.seek(SeekFrom::Start(offset))?;
    let record_wire = open_frame(reader, AadDomain::Record, keyring)?;
    decode_record(&record_wire)
}

pub fn read_record_payload(
    reader: &mut dyn ReadSeek,
    offset: u64,
    keyring: &Keyring,
) -> Result<Vec<u8>> {
    read_record(reader, offset, keyring)?;
    open_frame(reader, AadDomain::Payload, keyring)
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

    fn next_internal(&mut self) -> Result<Option<(u64, Record, Vec<u8>)>> {
        if self.current_offset == 0 {
            return Ok(None);
        }

        let offset = self.current_offset;
        let record = read_record(self.reader, offset, self.keyring)?;
        let payload = open_frame(self.reader, AadDomain::Payload, self.keyring)?;

        self.current_offset = record.prev_record_offset;
        Ok(Some((offset, record, payload)))
    }
}

impl<'a> Iterator for RecordIterator<'a> {
    type Item = Result<(u64, Record, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_internal().transpose()
    }
}
