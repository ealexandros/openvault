use std::io::SeekFrom;

use crate::errors::Result;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::traits::{ReadSeek, WriteSeek};
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};
use crate::vault::versions::v1::io::subheader::{read_subheader_from_rw, write_subheader};

pub fn read_checkpoint(
    reader: &mut dyn ReadSeek,
    offset: u64,
    keyring: &Keyring,
) -> Result<Vec<u8>> {
    reader.seek(SeekFrom::Start(offset))?;
    open_frame(reader, AadDomain::Checkpoint, keyring)
}

pub fn write_checkpoint(
    writer: &mut dyn WriteSeek,
    payload: &[u8],
    keyring: &Keyring,
) -> Result<u64> {
    let mut subheader = read_subheader_from_rw(writer, keyring)?;
    let offset = seal_frame(writer, AadDomain::Checkpoint, payload, keyring)?;

    subheader.checkpoint_offset = offset;
    write_subheader(writer, &subheader, keyring)?;

    Ok(offset)
}
