use std::io::{Read, Seek, Write};

use crate::errors::Result;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::record::Record;
use crate::vault::versions::shared::subheader::Subheader;

pub trait ReadSeek: Read + Seek {}

impl<T: Read + Seek> ReadSeek for T {}

pub trait WriteSeek: Read + Write + Seek {}

impl<T: Read + Write + Seek> WriteSeek for T {}

pub trait VersionHandler {
    fn version(&self) -> u16;

    fn init_layout(&self, writer: &mut dyn WriteSeek, keyring: &Keyring) -> Result<Subheader>;

    fn read_subheader(&self, reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result<Subheader>;

    fn write_subheader(
        &self,
        writer: &mut dyn WriteSeek,
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
        writer: &mut dyn WriteSeek,
        payload: &[u8],
        keyring: &Keyring,
    ) -> Result<u64>;

    fn append_record(
        &self,
        writer: &mut dyn WriteSeek,
        record: &Record,
        payload: &[u8],
        keyring: &Keyring,
    ) -> Result<u64>;

    fn read_record(
        &self,
        reader: &mut dyn ReadSeek,
        offset: u64,
        keyring: &Keyring,
    ) -> Result<Record>;

    fn read_record_payload(
        &self,
        reader: &mut dyn ReadSeek,
        record_offset: u64,
        keyring: &Keyring,
    ) -> Result<Vec<u8>>;

    /// Replays records from `start_offset` and performs an action (e.g. printing or rebuilding state).
    fn replay_from(
        &self,
        reader: &mut dyn ReadSeek,
        start_offset: u64,
        keyring: &Keyring,
    ) -> Result;

    /// Rewrites the vault into `writer` and returns the resulting subheader.
    fn compact(
        &self,
        reader: &mut dyn ReadSeek,
        writer: &mut dyn WriteSeek,
        keyring: &Keyring,
    ) -> Result<Subheader>;
}
