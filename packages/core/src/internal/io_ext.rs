use std::io::Result;
use std::io::{Read, Seek, SeekFrom, Write};

pub trait ReadExt: Read {
    fn read_exact_vec(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; len];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

impl<R: Read + ?Sized> ReadExt for R {}

pub trait SeekExt: Seek {
    fn seek_to_start(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Start(0))
    }

    fn seek_to_end(&mut self) -> Result<u64> {
        self.seek(SeekFrom::End(0))
    }

    fn seek_from_start(&mut self, offset: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(offset))
    }

    fn seek_from_end(&mut self, offset: i64) -> Result<u64> {
        self.seek(SeekFrom::End(offset))
    }
}

impl<T: Seek + ?Sized> SeekExt for T {}

pub trait ReadSeek: Read + Seek {}

impl<R: Read + Seek> ReadSeek for R {}

pub trait WriteSeek: Write + Seek {}

impl<W: Write + Seek> WriteSeek for W {}

pub trait ReadWriteSeek: ReadSeek + WriteSeek {}

impl<T: ReadSeek + WriteSeek> ReadWriteSeek for T {}

pub type Reader = dyn ReadSeek;
pub type Writer = dyn WriteSeek;
pub type ReadWriter = dyn ReadWriteSeek;
