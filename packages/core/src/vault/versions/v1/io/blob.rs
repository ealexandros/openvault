use std::io::SeekFrom;

use crate::errors::Result;
use crate::internal::io_ext::{Reader, Rw};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};

pub fn read_blob_at(reader: &mut Reader, offset: u64, keyring: &Keyring) -> Result<Vec<u8>> {
    reader.seek(SeekFrom::Start(offset))?;
    open_frame(reader, AadDomain::Blob, keyring)
}

pub fn write_blob(rw: &mut Rw, blob: &[u8], keyring: &Keyring) -> Result<u64> {
    rw.seek(SeekFrom::End(0))?;
    let offset = seal_frame(rw, AadDomain::Blob, blob, keyring)?;
    Ok(offset)
}
