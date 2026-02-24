use crate::errors::Result;
use crate::internal::io_ext::{ReadWrite, Reader};
use crate::vault::crypto::envelope::Envelope;
use crate::vault::versions::shared::frame::{read_frame, write_frame};
use crate::vault::versions::shared::traits::FormatContext;
use crate::vault::versions::v1::io::aad::AadDomain;
use openvault_crypto::encryption::Nonce;

pub fn seal_frame(
    rw: &mut ReadWrite,
    domain: AadDomain,
    data: &[u8],
    context: &FormatContext,
) -> Result<u64> {
    let offset = rw.stream_position()?;

    let nonce = Nonce::random();
    let aad = domain.encode(offset);
    let key = domain.derive_key(context.keyring)?;

    let envelope = Envelope::new(context.compressor, context.cipher);
    let ciphertext = envelope.seal_bytes(data, &key, &nonce, &aad)?;

    write_frame(rw, &nonce, &ciphertext)?;

    Ok(offset)
}

pub fn open_frame(
    reader: &mut Reader,
    domain: AadDomain,
    context: &FormatContext,
) -> Result<Vec<u8>> {
    let offset = reader.stream_position()?;

    let (frame, ciphertext) = read_frame(reader)?;

    let aad = domain.encode(offset);
    let key = domain.derive_key(context.keyring)?;

    let envelope = Envelope::new(context.compressor, context.cipher);

    envelope.open_bytes(&ciphertext, &key, &frame.nonce, &aad)
}
