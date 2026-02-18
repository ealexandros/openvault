use std::io::SeekFrom;

use openvault_crypto::encryption::Nonce;

use crate::errors::Result;
use crate::vault::crypto::envelope::Envelope;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::frame::{read_frame, write_frame};
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::{encode_aad, AadDomain};
use crate::vault::versions::v1::io::subheader::{read_subheader_from_rw, write_subheader};

pub fn read_checkpoint(
    reader: &mut dyn ReadSeek,
    offset: u64,
    keyring: &Keyring,
) -> Result<Vec<u8>> {
    reader.seek(SeekFrom::Start(offset))?;

    let (frame, ciphertext) = read_frame(reader)?;
    let envelope = Envelope::default();

    let envelope_key = keyring.envelope_key_bytes();
    let aad = encode_aad(AadDomain::Checkpoint, offset);

    envelope.open_bytes(&ciphertext, envelope_key, &frame.nonce, &aad)
}

pub fn write_checkpoint(
    writer: &mut dyn WriteSeek,
    payload: &[u8],
    keyring: &Keyring,
) -> Result<u64> {
    let mut subheader = read_subheader_from_rw(writer, keyring)?;
    let offset = writer.seek(SeekFrom::End(0))?;

    let nonce = Nonce::random();
    let envelope = Envelope::default();

    let envelope_key = keyring.envelope_key_bytes();
    let aad = encode_aad(AadDomain::Checkpoint, offset);

    let ciphertext = envelope.seal_bytes(payload, envelope_key, &nonce, &aad)?;

    write_frame(writer, &nonce, &ciphertext)?;
    subheader.checkpoint_offset = offset;
    write_subheader(writer, &subheader, keyring)?;

    Ok(offset)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::vault::crypto::keyring::Keyring;
    use crate::vault::features::FeatureType;
    use crate::vault::versions::shared::record::{Record, RecordKind};

    use super::{read_checkpoint, write_checkpoint};
    use crate::vault::versions::v1::io::init_layout;
    use crate::vault::versions::v1::io::record::{append_record, read_record};
    use crate::vault::versions::v1::io::subheader::read_subheader;

    fn test_keyring() -> Keyring {
        let salt = openvault_crypto::keys::random_salt();
        Keyring::derive(b"test-password", &salt).expect("failed to derive test keyring")
    }

    #[test]
    fn checkpoint_and_record_offsets_persist_in_subheader() {
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
            payload_size: payload.len() as u32,
            key_epoch: 0,
        };

        let record_offset =
            append_record(&mut io, &record, &payload, &keyring).expect("append record");
        let checkpoint_payload = b"checkpoint".to_vec();
        let checkpoint_offset =
            write_checkpoint(&mut io, &checkpoint_payload, &keyring).expect("write checkpoint");

        let subheader = read_subheader(&mut io, &keyring).expect("read subheader");
        assert_eq!(subheader.delta_offset, record_offset);
        assert_eq!(subheader.checkpoint_offset, checkpoint_offset);

        let (decoded_record, decoded_payload) =
            read_record(&mut io, subheader.delta_offset, &keyring).expect("read record");
        assert_eq!(decoded_record, record);
        assert_eq!(decoded_payload, payload);

        let restored_checkpoint =
            read_checkpoint(&mut io, subheader.checkpoint_offset, &keyring).expect("checkpoint");
        assert_eq!(restored_checkpoint, checkpoint_payload);
    }
}
