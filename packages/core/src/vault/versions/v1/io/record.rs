use crate::errors::Result;
use crate::vault::crypto::envelope::Envelope;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::frame::{read_frame, write_frame};
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::{AadDomain, encode_aad};
use crate::vault::versions::v1::io::subheader::{read_subheader_from_rw, write_subheader};
use crate::vault::versions::v1::mapper::{decode_record, encode_record};
use openvault_crypto::encryption::Nonce;
use std::io::SeekFrom;

fn seal_frame(
    writer: &mut dyn WriteSeek,
    domain: AadDomain,
    data: &[u8],
    keyring: &Keyring,
) -> Result<u64> {
    let offset = writer.stream_position()?;
    let nonce = Nonce::random();
    let aad = encode_aad(domain, offset);
    let ciphertext =
        Envelope::default().seal_bytes(data, keyring.envelope_key_bytes(), &nonce, &aad)?;

    write_frame(writer, &nonce, &ciphertext)?;
    Ok(offset)
}

fn open_frame(reader: &mut dyn ReadSeek, domain: AadDomain, keyring: &Keyring) -> Result<Vec<u8>> {
    let offset = reader.stream_position()?;
    let (frame, ciphertext) = read_frame(reader)?;
    let aad = encode_aad(domain, offset);

    Envelope::default().open_bytes(
        &ciphertext,
        keyring.envelope_key_bytes(),
        &frame.nonce,
        &aad,
    )
}

pub fn append_record(
    writer: &mut dyn WriteSeek,
    record: &Record,
    payload: &[u8],
    keyring: &Keyring,
) -> Result<u64> {
    let mut subheader = read_subheader_from_rw(writer, keyring)?;
    writer.seek(SeekFrom::End(0))?;

    let record_wire = encode_record(record)?;
    let record_offset = seal_frame(writer, AadDomain::Record, &record_wire, keyring)?;

    seal_frame(writer, AadDomain::Payload, payload, keyring)?;

    subheader.tail_record_offset = record_offset;
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

        self.current_offset = record.prev_offset;
        Ok(Some((offset, record, payload)))
    }
}

impl<'a> Iterator for RecordIterator<'a> {
    type Item = Result<(u64, Record, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_internal().transpose()
    }
}
