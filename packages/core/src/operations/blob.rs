use crate::errors::Result;
use crate::features::blob_ref::BlobRef;
use crate::internal::io_ext::Reader;
use crate::vault::runtime::VaultSession;

pub fn put_blob(session: &mut VaultSession, source: &mut Reader) -> Result<BlobRef> {
    let format = session.format();
    let keyring = session.keyring().clone();
    let file = session.file_mut();

    format.write_blob(file, source, &keyring)
}

pub fn get_blob(session: &mut VaultSession, blob_ref: &BlobRef) -> Result<Vec<u8>> {
    let format = session.format();
    let keyring = session.keyring().clone();
    let file = session.file_mut();

    format.read_blob(file, blob_ref, &keyring)
}
