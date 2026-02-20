use crate::errors::Result;
use crate::internal::io_ext::{Reader, Rw, Writer};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::checkpoint::Checkpoint;
use crate::vault::versions::shared::record::RecordHeader;
use crate::vault::versions::shared::subheader::Subheader;

pub trait VersionHandler {
    fn version(&self) -> u16;

    fn init_layout(&self, rw: &mut Rw, keyring: &Keyring) -> Result<Subheader>;

    fn read_subheader(&self, reader: &mut Reader, keyring: &Keyring) -> Result<Subheader>;

    fn read_blob_at(&self, reader: &mut Reader, offset: u64, keyring: &Keyring) -> Result<Vec<u8>>;

    fn write_blob(&self, rw: &mut Rw, blob: &[u8], keyring: &Keyring) -> Result<u64>;

    fn write_subheader(&self, rw: &mut Rw, subheader: &Subheader, keyring: &Keyring) -> Result;

    fn read_checkpoint(
        &self,
        reader: &mut Reader,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<Checkpoint>;

    fn write_checkpoint(
        &self,
        rw: &mut Rw,
        checkpoint: &Checkpoint,
        keyring: &Keyring,
    ) -> Result<u64>;

    fn append_record(
        &self,
        rw: &mut Rw,
        record: &RecordHeader,
        payload: &[u8],
        keyring: &Keyring,
    ) -> Result<u64>;

    fn read_record(
        &self,
        reader: &mut Reader,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<(RecordHeader, Vec<u8>)>;

    fn replay(&self, reader: &mut Reader, keyring: &Keyring) -> Result;

    fn compact(&self, reader: &mut Reader, writer: &mut Writer, keyring: &Keyring) -> Result;
}
