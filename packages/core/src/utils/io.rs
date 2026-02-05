use std::io::{self, Read};

pub trait ReadExt: Read {
    fn read_exact_array<const N: usize>(&mut self) -> io::Result<[u8; N]>;
}

impl<R: Read + ?Sized> ReadExt for R {
    fn read_exact_array<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buf = [0u8; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}
