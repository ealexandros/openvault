use std::io::Read;

use openvault_crypto::compression::CompressionAlgorithm;
use openvault_crypto::encryption::EncryptionAlgorithm;

use crate::errors::Result;
use crate::features::shared::blob_ref::BlobRef;
use crate::internal::io_ext::{ReadWriter, Reader, Writer};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::checkpoint::Checkpoint;
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::replay::ReplayState;
use crate::vault::versions::shared::subheader::Subheader;

pub struct FormatContext<'a> {
    pub keyring: &'a Keyring,
    pub compressor: CompressionAlgorithm,
    pub cipher: EncryptionAlgorithm,
}

impl<'a> FormatContext<'a> {
    pub fn new(
        keyring: &'a Keyring,
        compressor: CompressionAlgorithm,
        cipher: EncryptionAlgorithm,
    ) -> Self {
        Self {
            keyring,
            compressor,
            cipher,
        }
    }
}

pub trait FormatHandler: Sync + Send {
    fn version(&self) -> u16;

    fn init_layout(&self, rw: &mut ReadWriter, context: &FormatContext) -> Result<Subheader>;

    fn read_subheader(&self, reader: &mut Reader, context: &FormatContext) -> Result<Subheader>;

    fn write_subheader(
        &self,
        rw: &mut ReadWriter,
        subheader: &Subheader,
        context: &FormatContext,
    ) -> Result;

    fn read_blob(
        &self,
        reader: &mut Reader,
        blob_ref: &BlobRef,
        context: &FormatContext,
    ) -> Result<Vec<u8>>;

    fn write_blob(
        &self,
        rw: &mut ReadWriter,
        reader: &mut dyn Read,
        context: &FormatContext,
    ) -> Result<BlobRef>;

    fn read_checkpoint(
        &self,
        reader: &mut Reader,
        offset: u64,
        context: &FormatContext,
    ) -> Result<Checkpoint>;

    fn write_checkpoint(
        &self,
        rw: &mut ReadWriter,
        checkpoint: &mut Checkpoint,
        context: &FormatContext,
    ) -> Result<u64>;

    fn read_record(
        &self,
        reader: &mut Reader,
        offset: u64,
        context: &FormatContext,
    ) -> Result<Record>;

    fn append_record(
        &self,
        rw: &mut ReadWriter,
        record: &mut Record,
        context: &FormatContext,
    ) -> Result<u64>;

    fn replay(&self, reader: &mut Reader, context: &FormatContext) -> Result<ReplayState>;

    fn compact(&self, reader: &mut Reader, writer: &mut Writer, context: &FormatContext) -> Result;
}
