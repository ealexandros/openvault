use crate::errors::Result;
use crate::internal::io_ext::{Reader, Rw, SeekExt};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::checkpoint::Checkpoint;
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};
use crate::vault::versions::v1::io::subheader::{read_subheader, write_subheader};
use crate::vault::versions::v1::mapper::{decode_checkpoint, encode_checkpoint};

pub fn read_checkpoint(reader: &mut Reader, offset: u64, keyring: &Keyring) -> Result<Checkpoint> {
    reader.seek_from_start(offset)?;
    let checkpoint_bytes = open_frame(reader, AadDomain::Checkpoint, keyring)?;
    decode_checkpoint(&checkpoint_bytes)
}

pub fn write_checkpoint(
    rw: &mut Rw,
    checkpoint: &mut Checkpoint,
    keyring: &Keyring,
) -> Result<u64> {
    let mut subheader = read_subheader(rw, keyring)?;

    rw.seek_end()?;

    checkpoint.last_delta_sequence = subheader.last_sequence;

    let checkpoint_bytes = encode_checkpoint(&checkpoint)?;
    let offset = seal_frame(rw, AadDomain::Checkpoint, &checkpoint_bytes, keyring)?;

    subheader.checkpoint_offset = offset;
    write_subheader(rw, &subheader, keyring)?;

    Ok(offset)
}
