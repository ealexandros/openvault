use std::io::SeekFrom;

use openvault_crypto::encryption::Nonce;
use openvault_crypto::encryption::factory::EncryptionAlgorithm;

use crate::errors::Result;
use crate::vault::boot_header::VAULT_TOTAL_SIZE;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::frame::{read_frame, write_frame};
use crate::vault::versions::shared::subheader::Subheader;
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::{AadDomain, encode_aad};

pub const SUBHEADER_OFFSET: u64 = VAULT_TOTAL_SIZE as u64;

pub fn write_subheader(writer: &mut dyn WriteSeek, data: &Subheader, keyring: &Keyring) -> Result {
    let cipher = EncryptionAlgorithm::default().get()?;

    let nonce = Nonce::random();
    let aad = encode_aad(AadDomain::Subheader, SUBHEADER_OFFSET);
    let key = keyring.envelope_key_bytes();

    let ciphertext = cipher.encrypt(key, &nonce, &data.to_bytes()?, &aad)?;

    writer.seek(SeekFrom::Start(SUBHEADER_OFFSET))?;
    write_frame(writer, &nonce, &ciphertext)
}

pub fn read_subheader(reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result<Subheader> {
    reader.seek(SeekFrom::Start(SUBHEADER_OFFSET))?;

    let (frame, ciphertext) = read_frame(reader)?;

    let cipher = EncryptionAlgorithm::default().get()?;
    let aad = encode_aad(AadDomain::Subheader, SUBHEADER_OFFSET);
    let key = keyring.envelope_key_bytes();

    let plaintext = cipher.decrypt(key, &frame.nonce, &ciphertext, &aad)?;

    Subheader::from_bytes(&plaintext)
}

pub fn read_subheader_from_rw(rw: &mut dyn WriteSeek, keyring: &Keyring) -> Result<Subheader> {
    rw.seek(SeekFrom::Start(SUBHEADER_OFFSET))?;

    let (frame, ciphertext) = read_frame(rw)?;

    let cipher = EncryptionAlgorithm::default().get()?;
    let aad = encode_aad(AadDomain::Subheader, SUBHEADER_OFFSET);
    let key = keyring.envelope_key_bytes();

    let plaintext = cipher.decrypt(key, &frame.nonce, &ciphertext, &aad)?;

    Subheader::from_bytes(&plaintext)
}
