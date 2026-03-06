use crate::errors::Result;
use crate::internal::io_ext::{Reader, Writer};
use crate::vault::versions::shared::format::FormatContext;

pub fn run_compaction(
    _reader: &mut Reader,
    _writer: &mut Writer,
    _context: &FormatContext,
) -> Result {
    Ok(())
}
