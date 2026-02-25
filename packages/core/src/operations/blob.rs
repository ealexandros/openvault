use std::io::Read;

use crate::errors::Result;
use crate::features::shared::blob_ref::BlobRef;
use crate::vault::runtime::VaultSession;

pub fn put_blob(session: &mut VaultSession, source: &mut dyn Read) -> Result<BlobRef> {
    let format = session.format();

    session.with_format_context(|file, context| format.write_blob(file, source, context))
}

pub fn get_blob(session: &mut VaultSession, blob_ref: &BlobRef) -> Result<Vec<u8>> {
    let format = session.format();

    session.with_format_context(|file, context| format.read_blob(file, blob_ref, context))
}
