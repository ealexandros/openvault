use crate::errors::Result;
use crate::internal::io_ext::ReadSeek;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::v1::io;
use crate::vault::versions::v1::io::record::RecordIterator;

pub fn replay_state(reader: &mut dyn ReadSeek, keyring: &Keyring) -> Result {
    let subheader = io::read_subheader(reader, &keyring)?;

    for result in RecordIterator::new(reader, subheader.tail_record_offset, keyring) {
        let (_offset, record, _payload) = result?;
        println!("{:#?}", record);
    }

    Ok(())
}
