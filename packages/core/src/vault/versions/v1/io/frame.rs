use crate::errors::Result;
use crate::vault::crypto::envelope::Envelope;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::frame::{read_frame, write_frame};
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::{AadDomain, encode_aad};
use openvault_crypto::encryption::Nonce;

pub fn seal_frame(
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

pub fn open_frame(
    reader: &mut dyn ReadSeek,
    domain: AadDomain,
    keyring: &Keyring,
) -> Result<Vec<u8>> {
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
