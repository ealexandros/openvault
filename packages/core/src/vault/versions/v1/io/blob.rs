use crate::errors::Result;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};

pub fn read_blob(reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result<Vec<u8>> {
    open_frame(reader, AadDomain::Blob, keyring)
}

pub fn write_blob(writer: &mut dyn WriteSeek, blob: &[u8], keyring: &Keyring) -> Result {
    seal_frame(writer, AadDomain::Blob, blob, keyring)?;
    Ok(())
}
