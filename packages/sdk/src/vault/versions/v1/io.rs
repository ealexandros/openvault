use std::io::{Read, Seek, Write};

use crate::errors::Result;
use crate::versions::v1::records::Record;
use crate::versions::v1::schema::index::VaultIndex;

// pub struct V1Reader;

// impl V1Reader {
//     pub fn read_header<P: AsRef<Path>>(_path: P) -> Result<VaultIndex> {
//         todo!()
//     }

//     pub fn read_metadata<P: AsRef<Path>>(
//         _path: P,
//         _header: &VaultIndex,
//         _key: &[u8],
//     ) -> Result<VaultMeta> {
//         todo!()
//     }
// }

// pub struct V1Writer;

// impl V1Writer {
//     pub fn write_vault<P: AsRef<Path>>(
//         _path: P,
//         _header: &VaultIndex,
//         _meta: &VaultMeta,
//         _key: &[u8],
//     ) -> Result<()> {
//         todo!()
//     }
// }

/// Read the VaultIndex (right after the shared header)
pub fn read_index<R: Read + Seek>(_reader: &mut R) -> Result<VaultIndex> {
    todo!()
}

/// Write/update the VaultIndex
pub fn write_index<W: Write + Seek>(_writer: &mut W, _index: &VaultIndex) -> Result {
    todo!()
}

/// Read a single record at a given offset (returns record header + raw encrypted payload)
pub fn read_record<R: Read + Seek>(_reader: &mut R, _offset: u64) -> Result<(Record, Vec<u8>)> {
    todo!()
}

/// Append a record to the end of the file, return offset where it was written
pub fn append_record<W: Write + Seek>(
    _writer: &mut W,
    _record: &Record,
    _payload: &[u8],
) -> Result<u64> {
    todo!()
}

/// Walk the delta chain backward from `last_delta_offset`, return all payloads in forward order
pub fn read_delta_chain<R: Read + Seek>(
    _reader: &mut R,
    _last_delta_offset: u64,
) -> Result<Vec<Vec<u8>>> {
    todo!()
}

/// Read the snapshot payload at the given offset
pub fn read_snapshot<R: Read + Seek>(_reader: &mut R, _snapshot_offset: u64) -> Result<Vec<u8>> {
    todo!()
}

/// Rewrite the vault with only the latest snapshot, removing old deltas
pub fn compact<R: Read + Write + Seek>(_file: &mut R, _key: &[u8]) -> Result<()> {
    todo!()
}
