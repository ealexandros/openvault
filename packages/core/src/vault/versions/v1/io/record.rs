use std::io::SeekFrom;

use openvault_crypto::encryption::Nonce;

use crate::errors::{Error, Result};
use crate::vault::crypto::envelope::Envelope;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::frame::{FrameHeader, read_frame, write_frame};
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::{AadDomain, encode_aad};
use crate::vault::versions::v1::io::subheader::{read_subheader_from_rw, write_subheader};
use crate::vault::versions::v1::mapper::{decode_record, encode_record};

pub fn append_record(
    writer: &mut dyn WriteSeek,
    record: &Record,
    payload: &[u8],
    keyring: &Keyring,
) -> Result<u64> {
    let mut subheader = read_subheader_from_rw(writer, keyring)?;
    let offset = writer.seek(SeekFrom::End(0))?;

    let envelope = Envelope::default();
    let envelope_key = keyring.envelope_key_bytes();

    let record_nonce = Nonce::random();
    let record_aad = encode_aad(AadDomain::Record, offset);
    let record_wire = encode_record(record)?;
    let record_ciphertext =
        envelope.seal_bytes(&record_wire, envelope_key, &record_nonce, &record_aad)?;
    write_frame(writer, &record_nonce, &record_ciphertext)?;

    let payload_offset = writer.seek(SeekFrom::Current(0))?;
    let payload_nonce = Nonce::random();
    let payload_aad = encode_aad(AadDomain::Payload, payload_offset);
    let payload_ciphertext =
        envelope.seal_bytes(payload, envelope_key, &payload_nonce, &payload_aad)?;
    write_frame(writer, &payload_nonce, &payload_ciphertext)?;

    subheader.delta_offset = offset;
    write_subheader(writer, &subheader, keyring)?;

    Ok(offset)
}

pub fn read_record(reader: &mut dyn ReadSeek, offset: u64, keyring: &Keyring) -> Result<Record> {
    reader.seek(SeekFrom::Start(offset))?;

    let envelope = Envelope::default();
    let envelope_key = keyring.envelope_key_bytes();

    let (header_frame, record_ciphertext) = read_frame(reader)?;
    let record_aad = encode_aad(AadDomain::Record, offset);
    let record_wire = envelope.open_bytes(
        &record_ciphertext,
        envelope_key,
        &header_frame.nonce,
        &record_aad,
    )?;

    decode_record(&record_wire)
}

pub fn read_record_payload(
    reader: &mut dyn ReadSeek,
    record_offset: u64,
    keyring: &Keyring,
) -> Result<Vec<u8>> {
    reader.seek(SeekFrom::Start(record_offset))?;

    let record_frame = FrameHeader::read_from(reader)?;

    reader.seek(SeekFrom::Current(record_frame.size.into()))?;

    let payload_offset = reader.stream_position()?;
    let payload_aad = encode_aad(AadDomain::Payload, payload_offset);

    let (payload_frame, payload_ciphertext) = read_frame(reader)?;

    let envelope = Envelope::default();
    let envelope_key = keyring.envelope_key_bytes();

    envelope.open_bytes(
        &payload_ciphertext,
        envelope_key,
        &payload_frame.nonce,
        &payload_aad,
    )
}

pub fn replay_from(
    reader: &mut dyn ReadSeek,
    start_offset: u64,
    keyring: &Keyring,
) -> Result<Vec<(u64, Record, Vec<u8>)>> {
    let end_offset = reader.seek(SeekFrom::End(0))?;

    if start_offset > end_offset {
        return Err(Error::InvalidVaultFormat);
    }

    reader.seek(SeekFrom::Start(start_offset))?;

    let envelope = Envelope::default();
    let envelope_key = keyring.envelope_key_bytes();
    let mut out = Vec::new();

    while reader.seek(SeekFrom::Current(0))? < end_offset {
        let record_offset = reader.seek(SeekFrom::Current(0))?;

        // 1. Read Record Header
        let (header_frame, record_ciphertext) = read_frame(reader)?;
        let record_aad = encode_aad(AadDomain::Record, record_offset);
        let record_wire = envelope.open_bytes(
            &record_ciphertext,
            envelope_key,
            &header_frame.nonce,
            &record_aad,
        )?;
        let record = decode_record(&record_wire)?;

        // 2. Read Payload
        let payload_offset = reader.seek(SeekFrom::Current(0))?;
        let (payload_frame, payload_ciphertext) = read_frame(reader)?;
        let payload_aad = encode_aad(AadDomain::Payload, payload_offset);
        let payload = envelope.open_bytes(
            &payload_ciphertext,
            envelope_key,
            &payload_frame.nonce,
            &payload_aad,
        )?;

        out.push((record_offset, record, payload));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::vault::crypto::keyring::Keyring;
    use crate::vault::features::FeatureType;
    use crate::vault::versions::shared::record::{Record, RecordKind};
    use crate::vault::versions::v1::io::init_layout;

    use super::{append_record, read_record, read_record_payload, replay_from};

    fn test_keyring() -> Keyring {
        let salt = openvault_crypto::keys::random_salt();
        Keyring::derive(b"test-password", &salt).expect("failed to derive test keyring")
    }

    #[test]
    fn append_and_read_record_roundtrip() {
        let keyring = test_keyring();
        let mut io = Cursor::new(Vec::new());
        init_layout(&mut io, &keyring).expect("init layout");

        let payload = b"record-payload".to_vec();
        let record = Record {
            kind: RecordKind::Delta,
            feature_id: FeatureType::Secrets,
            payload_version: 1,
            sequence: 1,
            prev_offset: 0,
            key_epoch: 0,
        };

        let offset =
            append_record(&mut io, &record, &payload, &keyring).expect("append should succeed");
        let decoded_record =
            read_record(&mut io, offset, &keyring).expect("read record should succeed");
        let decoded_payload =
            read_record_payload(&mut io, offset, &keyring).expect("read payload should succeed");

        assert_eq!(decoded_record, record);
        assert_eq!(decoded_payload, payload);
    }

    #[test]
    fn replay_reads_all_records_from_offset() {
        let keyring = test_keyring();
        let mut io = Cursor::new(Vec::new());
        init_layout(&mut io, &keyring).expect("init layout");

        let payload_a = b"A".to_vec();
        let payload_b = b"BB".to_vec();

        let record_a = Record {
            kind: RecordKind::Delta,
            feature_id: FeatureType::Secrets,
            payload_version: 1,
            sequence: 1,
            prev_offset: 0,
            key_epoch: 0,
        };

        let record_b = Record {
            kind: RecordKind::Delta,
            feature_id: FeatureType::Secrets,
            payload_version: 1,
            sequence: 2,
            prev_offset: 0,
            key_epoch: 0,
        };

        let offset_a =
            append_record(&mut io, &record_a, &payload_a, &keyring).expect("first append");
        let offset_b =
            append_record(&mut io, &record_b, &payload_b, &keyring).expect("second append");

        let replayed = replay_from(&mut io, offset_a, &keyring).expect("replay should succeed");

        assert_eq!(replayed.len(), 2);
        assert_eq!(replayed[0].0, offset_a);
        assert_eq!(replayed[1].0, offset_b);
        assert_eq!(replayed[0].1, record_a);
        assert_eq!(replayed[1].1, record_b);
        assert_eq!(replayed[0].2, payload_a);
        assert_eq!(replayed[1].2, payload_b);
    }
}
