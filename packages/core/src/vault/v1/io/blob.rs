use std::fs::File;
use std::io::Seek;
use std::path::Path;

use crate::errors::Result;
use crate::vault::v1::io::IoContext;
use crate::vault::v1::schema::entries::FileMeta;
use crate::vault::v1::schema::header::VAULT_HEADER_SIZE;

pub fn write_file(
    file_meta: &mut FileMeta,
    source_dir: &Path,
    output_file: &mut File,
    key: &[u8],
    ctx: &IoContext,
) -> Result {
    let full_path = source_dir.join(file_meta.relative_path.as_path());
    let mut input = File::open(full_path)?;

    let start = output_file.stream_position()?;

    let mut buf = Vec::new();

    ctx.compressor()?.compress_stream(&mut input, &mut buf)?;
    ctx.cipher.encrypt_stream(key, &mut &buf[..], output_file)?;

    let end = output_file.stream_position()?;

    file_meta.blob.offset = start - (VAULT_HEADER_SIZE as u64);
    file_meta.blob.size = end - start;

    Ok(())
}
