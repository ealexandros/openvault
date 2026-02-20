use crate::errors::Result;
use crate::internal::io_ext::Reader;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::v1::io;
use crate::vault::versions::v1::io::record::replay_records;

pub fn replay_state(reader: &mut Reader, keyring: &Keyring) -> Result {
    let subheader = io::read_subheader(reader, keyring)?;

    let records = replay_records(reader, subheader.tail_record_offset, keyring)?;

    for (_offset, record) in records {
        println!("{:#?}", record.header);
        println!("{:#?}", record.payload);
    }

    Ok(())
}
