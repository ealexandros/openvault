pub mod checkpoint;
pub mod io;
pub mod mapper;
pub mod replay;

use crate::errors::{Error, Result};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::record::RecordHeader;
use crate::vault::versions::shared::subheader::Subheader;
use crate::vault::versions::shared::traits::{ReadSeek, VersionHandler, WriteSeek};

#[derive(Debug, Default)]
pub struct V1Handler;

impl VersionHandler for V1Handler {
    fn version(&self) -> u16 {
        1
    }

    fn init_layout(&self, writer: &mut dyn WriteSeek, keyring: &Keyring) -> Result<Subheader> {
        io::init_layout(writer, keyring)
    }

    fn read_subheader(&self, reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result<Subheader> {
        io::read_subheader(reader, keyring)
    }

    fn read_blob(&self, reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result<Vec<u8>> {
        io::read_blob(reader, keyring)
    }

    fn write_blob(&self, writer: &mut dyn WriteSeek, blob: &[u8], keyring: &Keyring) -> Result {
        io::write_blob(writer, blob, keyring)
    }

    fn write_subheader(
        &self,
        writer: &mut dyn WriteSeek,
        subheader: &Subheader,
        keyring: &Keyring,
    ) -> Result {
        io::write_subheader(writer, subheader, keyring)
    }

    fn read_checkpoint(
        &self,
        reader: &mut dyn ReadSeek,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<Vec<u8>> {
        io::read_checkpoint(reader, offset, keyring)
    }

    fn write_checkpoint(
        &self,
        writer: &mut dyn WriteSeek,
        payload: &[u8],
        keyring: &Keyring,
    ) -> Result<u64> {
        io::write_checkpoint(writer, payload, keyring)
    }

    fn append_record(
        &self,
        writer: &mut dyn WriteSeek,
        record: &RecordHeader,
        payload_plaintext: &[u8],
        keyring: &Keyring,
    ) -> Result<u64> {
        io::append_record(writer, record, payload_plaintext, keyring)
    }

    fn read_record(
        &self,
        reader: &mut dyn ReadSeek,
        record_offset: u64,
        keyring: &Keyring,
    ) -> Result<(RecordHeader, Vec<u8>)> {
        io::read_record(reader, record_offset, keyring)
    }

    fn replay(&self, reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result {
        replay::replay_state(reader, keyring)
    }

    fn compact(
        &self,
        _reader: &mut dyn ReadSeek,
        _writer: &mut dyn WriteSeek,
        _keyring: &Keyring,
    ) -> Result<Subheader> {
        Err(Error::InvalidVaultFormat)
    }
}
