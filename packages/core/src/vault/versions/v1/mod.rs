pub mod blob;
pub mod io;
pub mod mapper;
pub mod replay;

use std::io::Read;

use crate::errors::{Error, Result};
use crate::features::blob_ref::BlobRef;
use crate::internal::io_ext::{ReadWrite, Reader, Writer};
use crate::vault::versions::shared::checkpoint::Checkpoint;
use crate::vault::versions::shared::record::RecordHeader;
use crate::vault::versions::shared::subheader::Subheader;
use crate::vault::versions::shared::traits::{FormatContext, FormatHandler};
use crate::vault::versions::v1::replay::replay_records;

pub const V1_FORMAT_VERSION: u16 = 1;

#[derive(Debug, Default)]
pub struct V1FormatHandler;

impl FormatHandler for V1FormatHandler {
    fn version(&self) -> u16 {
        V1_FORMAT_VERSION
    }

    fn init_layout(&self, rw: &mut ReadWrite, context: &FormatContext) -> Result<Subheader> {
        io::init_layout(rw, context)
    }

    fn read_subheader(&self, reader: &mut Reader, context: &FormatContext) -> Result<Subheader> {
        io::read_subheader(reader, context)
    }

    fn read_blob(
        &self,
        reader: &mut Reader,
        blob_ref: &BlobRef,
        context: &FormatContext,
    ) -> Result<Vec<u8>> {
        io::read_blob(reader, blob_ref, context)
    }

    fn write_blob(
        &self,
        rw: &mut ReadWrite,
        reader: &mut dyn Read,
        context: &FormatContext,
    ) -> Result<BlobRef> {
        io::write_blob(rw, reader, context)
    }

    fn write_subheader(
        &self,
        rw: &mut ReadWrite,
        subheader: &Subheader,
        context: &FormatContext,
    ) -> Result {
        io::write_subheader(rw, subheader, context)
    }

    fn read_checkpoint(
        &self,
        reader: &mut Reader,
        offset: u64,
        context: &FormatContext,
    ) -> Result<Checkpoint> {
        io::read_checkpoint(reader, offset, context)
    }

    fn write_checkpoint(
        &self,
        rw: &mut ReadWrite,
        checkpoint: &mut Checkpoint,
        context: &FormatContext,
    ) -> Result<u64> {
        io::write_checkpoint(rw, checkpoint, context)
    }

    fn append_record(
        &self,
        rw: &mut ReadWrite,
        record: &RecordHeader,
        payload: &[u8],
        context: &FormatContext,
    ) -> Result<u64> {
        io::append_record(rw, record, payload, context)
    }

    fn read_record(
        &self,
        reader: &mut Reader,
        offset: u64,
        context: &FormatContext,
    ) -> Result<(RecordHeader, Vec<u8>)> {
        let record_wire = io::read_record(reader, offset, context)?;
        Ok((record_wire.header, record_wire.payload))
    }

    fn replay(&self, reader: &mut Reader, context: &FormatContext) -> Result {
        replay_records(reader, context)
    }

    fn compact(
        &self,
        _reader: &mut Reader,
        _writer: &mut Writer,
        _context: &FormatContext,
    ) -> Result {
        Err(Error::InvalidVaultFormat)
    }
}
