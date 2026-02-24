use crate::errors::Result;
use crate::internal::io_ext::Reader;
use crate::vault::versions::shared::replay::{ReplayRecord, ReplayState};
use crate::vault::versions::shared::traits::FormatContext;
use crate::vault::versions::v1::io::record::read_replay_records;
use crate::vault::versions::v1::io::{read_checkpoint, read_subheader};

pub fn replay_records(reader: &mut Reader, context: &FormatContext) -> Result<ReplayState> {
    let subheader = read_subheader(reader, context)?;

    let checkpoint = (subheader.checkpoint_offset != 0)
        .then(|| read_checkpoint(reader, subheader.checkpoint_offset, context))
        .transpose()?;

    let records = read_replay_records(
        reader,
        subheader.tail_record_offset,
        subheader.checkpoint_offset,
        context,
    )?;

    let records = records
        .into_iter()
        .map(|(offset, record)| ReplayRecord::new(offset, record.header, record.payload))
        .collect();

    Ok(ReplayState::new(subheader, checkpoint, records))
}
