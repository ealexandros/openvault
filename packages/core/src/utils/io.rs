use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Seek, SeekFrom, Write};

// @todo-now rethink this

/// Reads exactly `len` bytes from the reader.
///
/// # Warning
/// This function pre-allocates a vector of size `len`. If `len` is large or
/// controlled by untrusted input, this can lead to excessive memory usage or OOM.
/// For potentially large reads, consider streaming or reading in chunks.
pub fn read_exact_bytes<R: Read>(reader: &mut R, len: usize) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn write_bytes<W: Write>(writer: &mut W, data: &[u8]) -> io::Result<()> {
    writer.write_all(data)
}

pub fn read_u64_le<R: Read>(reader: &mut R) -> io::Result<u64> {
    reader.read_u64::<LittleEndian>()
}

pub fn write_u64_le<W: Write>(writer: &mut W, value: u64) -> io::Result<()> {
    writer.write_u64::<LittleEndian>(value)
}

pub fn read_u32_le<R: Read>(reader: &mut R) -> io::Result<u32> {
    reader.read_u32::<LittleEndian>()
}

pub fn write_u32_le<W: Write>(writer: &mut W, value: u32) -> io::Result<()> {
    writer.write_u32::<LittleEndian>(value)
}

pub fn seek<R: Seek>(reader: &mut R, pos: u64) -> io::Result<()> {
    reader.seek(SeekFrom::Start(pos))?;
    Ok(())
}

pub fn copy_stream<R: Read, W: Write, F: Fn(u64)>(
    reader: &mut R,
    writer: &mut W,
    buffer_size: usize,
    mut progress: Option<F>,
) -> io::Result<u64> {
    let mut buf = vec![0u8; buffer_size];
    let mut total = 0u64;

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        writer.write_all(&buf[..n])?;
        total += n as u64;

        if let Some(ref mut cb) = progress {
            cb(total);
        }
    }

    Ok(total)
}
