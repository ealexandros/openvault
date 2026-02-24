use crate::errors::Result;
use crate::internal::io_ext::Reader;
use crate::vault::versions::shared::traits::FormatContext;
use crate::vault::versions::v1::io::record::read_replay_records;
use crate::vault::versions::v1::io::{read_checkpoint, read_subheader};

pub fn replay_records(reader: &mut Reader, context: &FormatContext) -> Result {
    let subheader = read_subheader(reader, context)?;

    if subheader.checkpoint_offset != 0 {
        let checkpoint = read_checkpoint(reader, subheader.checkpoint_offset, context)?;
        println!("{:#?}", checkpoint);
    }

    let records = read_replay_records(
        reader,
        subheader.tail_record_offset,
        subheader.checkpoint_offset,
        context,
    )?;

    for (_offset, record) in records {
        println!("{:#?}", record.header);
        println!("{:#?}", record.payload);
    }

    Ok(())
}
