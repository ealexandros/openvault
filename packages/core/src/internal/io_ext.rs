use std::io::{self, Read, Seek, SeekFrom, Write};

pub trait ReadExt: Read {
    fn read_exact_vec(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<R: Read + ?Sized> ReadExt for R {}

pub trait SeekExt: Seek {
    fn seek_start(&mut self) -> io::Result<u64> {
        self.seek(SeekFrom::Start(0))
    }

    fn seek_end(&mut self) -> io::Result<u64> {
        self.seek(SeekFrom::End(0))
    }

    fn seek_from_start(&mut self, offset: u64) -> io::Result<u64> {
        self.seek(SeekFrom::Start(offset))
    }

    fn seek_from_end(&mut self, offset: i64) -> io::Result<u64> {
        self.seek(SeekFrom::End(offset))
    }
}

impl<S: Seek> SeekExt for S {}

pub trait ReadSeek: Read + Seek {}

impl<R: Read + Seek> ReadSeek for R {}

pub trait WriteSeek: Write + Seek {}

impl<W: Write + Seek> WriteSeek for W {}

pub trait ReadWrite: ReadSeek + WriteSeek {}

impl<R: Read + Write + Seek> ReadWrite for R {}

pub type Writer = dyn WriteSeek;

pub type Reader = dyn ReadSeek;

pub type Rw = dyn ReadWrite;
