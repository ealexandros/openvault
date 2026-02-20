use crate::errors::Result;
use crate::internal::io_ext::{ReadSeek, ReadWrite};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::record::RecordHeader;
use crate::vault::versions::shared::subheader::Subheader;

pub trait VersionHandler {
    fn version(&self) -> u16;

    fn init_layout(&self, writer: &mut dyn ReadWrite, keyring: &Keyring) -> Result<Subheader>;

    fn read_subheader(&self, reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result<Subheader>;

    fn read_blob_at(
        &self,
        reader: &mut dyn ReadSeek,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<Vec<u8>>;

    fn write_blob(&self, writer: &mut dyn ReadWrite, blob: &[u8], keyring: &Keyring)
    -> Result<u64>;

    fn write_subheader(
        &self,
        writer: &mut dyn ReadWrite,
        subheader: &Subheader,
        keyring: &Keyring,
    ) -> Result;

    fn read_checkpoint(
        &self,
        reader: &mut dyn ReadSeek,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<Vec<u8>>;

    fn write_checkpoint(
        &self,
        writer: &mut dyn ReadWrite,
        payload: &[u8],
        keyring: &Keyring,
    ) -> Result<u64>;

    fn append_record(
        &self,
        writer: &mut dyn ReadWrite,
        record: &RecordHeader,
        payload: &[u8],
        keyring: &Keyring,
    ) -> Result<u64>;

    fn read_record(
        &self,
        reader: &mut dyn ReadSeek,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<(RecordHeader, Vec<u8>)>;

    fn replay(&self, reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result;

    /// Rewrites the vault into `writer` and returns the resulting subheader.
    fn compact(
        &self,
        reader: &mut dyn ReadSeek,
        writer: &mut dyn ReadWrite,
        keyring: &Keyring,
    ) -> Result<Subheader>;
}
