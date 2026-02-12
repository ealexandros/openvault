use crate::versions::v1::schema::header::VaultHeader;
use crate::versions::v1::schema::metadata::VaultMeta;
use std::io::Result;
use std::path::Path;

pub struct V1Reader;

impl V1Reader {
    pub fn read_header<P: AsRef<Path>>(_path: P) -> Result<VaultHeader> {
        // @todo-now: Implement V1 header reading logic
        // Open file, seek to start, read 39 bytes, parse struct, verify magic/CRC
        todo!()
    }

    pub fn read_metadata<P: AsRef<Path>>(
        _path: P,
        _header: &VaultHeader,
        _key: &[u8],
    ) -> Result<VaultMeta> {
        // @todo-now: Implement V1 metadata reading logic
        // Seek to header.metadata_offset, read header.metadata_size, decrypt with provided key, deserialize via postcard
        todo!()
    }
}

pub struct V1Writer;

impl V1Writer {
    pub fn write_vault<P: AsRef<Path>>(
        _path: P,
        _header: &VaultHeader,
        _meta: &VaultMeta,
        _key: &[u8],
    ) -> Result<()> {
        // @todo-now: Implement V1 vault writing logic
        // Serialize metadata, compress, encrypt, append to file, update header with new offset/size, write header
        todo!()
    }
}
