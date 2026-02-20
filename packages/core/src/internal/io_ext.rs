use std::io::{self, Read, Seek, Write};

pub trait ReadExt: Read {
    fn read_exact_vec(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

pub trait ReadSeek: Read + Seek {}

pub trait WriteSeek: Write + Seek {}

pub trait ReadWrite: ReadSeek + WriteSeek {}

impl<R: Read + ?Sized> ReadExt for R {}

impl<R: Read + Seek> ReadSeek for R {}

impl<W: Write + Seek> WriteSeek for W {}

impl<R: Read + Write + Seek> ReadWrite for R {}

pub type Writer = dyn WriteSeek;

pub type Reader = dyn ReadSeek;

pub type Rw = dyn ReadWrite;
