use std::io::SeekFrom;

use crate::errors::Result;
use crate::internal::io_ext::{Reader, Rw};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};
use crate::vault::versions::v1::io::subheader::{read_subheader, write_subheader};

pub fn read_checkpoint(reader: &mut Reader, offset: u64, keyring: &Keyring) -> Result<Vec<u8>> {
    reader.seek(SeekFrom::Start(offset))?;
    open_frame(reader, AadDomain::Checkpoint, keyring)
}

pub fn write_checkpoint(rw: &mut Rw, payload: &[u8], keyring: &Keyring) -> Result<u64> {
    rw.seek(SeekFrom::End(0))?;

    let mut subheader = read_subheader(rw, keyring)?;
    let offset = seal_frame(rw, AadDomain::Checkpoint, payload, keyring)?;

    subheader.checkpoint_offset = offset;
    write_subheader(rw, &subheader, keyring)?;

    Ok(offset)
}
