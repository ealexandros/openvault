pub mod checkpoint;
pub mod io;
pub mod mapper;
pub mod replay;

use crate::errors::{Error, Result};
use crate::internal::io_ext::{Reader, Rw, Writer};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::record::RecordHeader;
use crate::vault::versions::shared::subheader::Subheader;
use crate::vault::versions::shared::traits::VersionHandler;

#[derive(Debug, Default)]
pub struct V1Handler;

impl VersionHandler for V1Handler {
    fn version(&self) -> u16 {
        1
    }

    fn init_layout(&self, rw: &mut Rw, keyring: &Keyring) -> Result<Subheader> {
        io::init_layout(rw, keyring)
    }

    fn read_subheader(&self, reader: &mut Reader, keyring: &Keyring) -> Result<Subheader> {
        io::read_subheader(reader, keyring)
    }

    fn read_blob_at(&self, reader: &mut Reader, offset: u64, keyring: &Keyring) -> Result<Vec<u8>> {
        io::read_blob_at(reader, offset, keyring)
    }

    fn write_blob(&self, rw: &mut Rw, blob: &[u8], keyring: &Keyring) -> Result<u64> {
        io::write_blob(rw, blob, keyring)
    }

    fn write_subheader(&self, rw: &mut Rw, subheader: &Subheader, keyring: &Keyring) -> Result {
        io::write_subheader(rw, subheader, keyring)
    }

    fn read_checkpoint(
        &self,
        reader: &mut Reader,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<Vec<u8>> {
        io::read_checkpoint(reader, offset, keyring)
    }

    fn write_checkpoint(&self, rw: &mut Rw, payload: &[u8], keyring: &Keyring) -> Result<u64> {
        io::write_checkpoint(rw, payload, keyring)
    }

    fn append_record(
        &self,
        rw: &mut Rw,
        record: &RecordHeader,
        payload: &[u8],
        keyring: &Keyring,
    ) -> Result<u64> {
        io::append_record(rw, record, payload, keyring)
    }

    fn read_record(
        &self,
        reader: &mut Reader,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<(RecordHeader, Vec<u8>)> {
        io::read_record(reader, offset, keyring)
    }

    fn replay(&self, reader: &mut Reader, keyring: &Keyring) -> Result {
        replay::replay_state(reader, keyring)
    }

    fn compact(
        &self,
        _reader: &mut Reader,
        _writer: &mut Writer,
        _keyring: &Keyring,
    ) -> Result<Subheader> {
        Err(Error::InvalidVaultFormat)
    }
}
