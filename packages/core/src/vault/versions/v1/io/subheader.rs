use openvault_crypto::encryption::Nonce;
use openvault_crypto::encryption::factory::EncryptionAlgorithm;

use crate::errors::Result;
use crate::internal::io_ext::{Reader, Rw, SeekExt};
use crate::vault::boot_header::VAULT_TOTAL_SIZE;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::frame::{read_frame, write_frame};
use crate::vault::versions::shared::subheader::Subheader;
use crate::vault::versions::v1::io::aad::AadDomain;

pub const SUBHEADER_OFFSET: u64 = VAULT_TOTAL_SIZE as u64;

pub fn write_subheader(rw: &mut Rw, data: &Subheader, keyring: &Keyring) -> Result {
    let cipher = EncryptionAlgorithm::default().get()?;

    let nonce = Nonce::random();
    let aad = AadDomain::Subheader.encode(SUBHEADER_OFFSET);
    let key = keyring.derive_meta_key()?;

    let ciphertext = cipher.encrypt(&key, &nonce, &data.to_bytes()?, &aad)?;

    rw.seek_from_start(SUBHEADER_OFFSET)?;

    write_frame(rw, &nonce, &ciphertext)
}

pub fn read_subheader(reader: &mut Reader, keyring: &Keyring) -> Result<Subheader> {
    reader.seek_from_start(SUBHEADER_OFFSET)?;

    let (frame, ciphertext) = read_frame(reader)?;

    let cipher = EncryptionAlgorithm::default().get()?;
    let aad = AadDomain::Subheader.encode(SUBHEADER_OFFSET);
    let key = keyring.derive_meta_key()?;

    let plaintext = cipher.decrypt(&key, &frame.nonce, &ciphertext, &aad)?;

    Subheader::from_bytes(&plaintext)
}
