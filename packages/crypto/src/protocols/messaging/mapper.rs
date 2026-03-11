use base64::Engine;
use base64::engine::general_purpose;

use crate::errors::{Error, Result};
use crate::protocols::messaging::MessageEnvelope;

const SIGNATURE_LEN_BYTES: usize = 2;

pub fn encode_payload(signature: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let sig_len = signature.len();

    if sig_len > u16::MAX as usize {
        return Err(Error::InvalidEnvelope);
    }

    let mut output = Vec::with_capacity(SIGNATURE_LEN_BYTES + sig_len + message.len());

    output.extend_from_slice(&(sig_len as u16).to_be_bytes());
    output.extend_from_slice(signature);
    output.extend_from_slice(message);

    Ok(output)
}

pub fn decode_payload(payload: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    if payload.len() < SIGNATURE_LEN_BYTES {
        return Err(Error::InvalidEnvelope);
    }

    let sig_len = u16::from_be_bytes([payload[0], payload[1]]) as usize;
    let signature_start = SIGNATURE_LEN_BYTES;
    let signature_end = signature_start + sig_len;

    if payload.len() < signature_end {
        return Err(Error::InvalidEnvelope);
    }

    let signature = payload[signature_start..signature_end].to_vec();
    let message = payload[signature_end..].to_vec();

    Ok((signature, message))
}

pub fn encode_message(message: &MessageEnvelope) -> Result<String> {
    let bytes = postcard::to_allocvec(message).map_err(|_| Error::InvalidMessageFormat)?;
    Ok(general_purpose::STANDARD.encode(bytes))
}

pub fn decode_message(bytes: &[u8]) -> Result<MessageEnvelope> {
    let decoded = general_purpose::STANDARD
        .decode(bytes)
        .map_err(|_| Error::DecodeBase64)?;

    postcard::from_bytes(&decoded).map_err(|_| Error::InvalidMessageFormat)
}
