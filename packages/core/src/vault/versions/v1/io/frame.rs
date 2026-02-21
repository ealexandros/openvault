use crate::errors::Result;
use crate::internal::io_ext::{Reader, Rw};
use crate::vault::crypto::envelope::Envelope;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::frame::{read_frame, write_frame};
use crate::vault::versions::v1::io::aad::AadDomain;
use openvault_crypto::encryption::Nonce;

pub fn seal_frame(rw: &mut Rw, domain: AadDomain, data: &[u8], keyring: &Keyring) -> Result<u64> {
    let offset = rw.stream_position()?;

    let nonce = Nonce::random();
    let aad = domain.encode(offset);
    let key = domain.derive_key(keyring)?;

    let ciphertext = Envelope::default().seal_bytes(data, &key, &nonce, &aad)?;

    write_frame(rw, &nonce, &ciphertext)?;

    Ok(offset)
}

pub fn open_frame(reader: &mut Reader, domain: AadDomain, keyring: &Keyring) -> Result<Vec<u8>> {
    let offset = reader.stream_position()?;

    let (frame, ciphertext) = read_frame(reader)?;

    let envelope = Envelope::default();
    let aad = domain.encode(offset);
    let key = domain.derive_key(keyring)?;

    envelope.open_bytes(&ciphertext, &key, &frame.nonce, &aad)
}
