use crate::errors::Result;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::Offset;
use crate::vault::versions::shared::checkpoint::Checkpoint;
use crate::vault::versions::shared::record::Record;

const CHECKPOINT_INTERVAL: u64 = 400;

pub fn append_record(session: &mut VaultSession, record: &mut Record) -> Result<Offset> {
    let format = session.format();

    session.with_format_context(|file, context| format.append_record(file, record, context))
}

pub fn create_checkpoint(session: &mut VaultSession, cp: &mut Checkpoint) -> Result<Offset> {
    let format = session.format();

    session.with_format_context(|file, context| format.write_checkpoint(file, cp, context))
}

pub fn should_create_checkpoint(session: &mut VaultSession) -> Result<bool> {
    let format = session.format();

    session.with_format_context(|file, context| {
        let subheader = format.read_subheader(file, context)?;

        let last_checkpoint_sequence = (subheader.checkpoint_offset != 0)
            .then(|| format.read_checkpoint(file, subheader.checkpoint_offset, context))
            .transpose()?
            .map_or(0, |checkpoint| checkpoint.sequence);

        let sequence_gap = subheader
            .last_sequence
            .saturating_sub(last_checkpoint_sequence);

        Ok(sequence_gap >= CHECKPOINT_INTERVAL)
    })
}

pub fn current_delta_sequence(session: &mut VaultSession) -> Result<u64> {
    let format = session.format();

    session.with_format_context(|file, context| {
        let subheader = format.read_subheader(file, context)?;
        Ok(subheader.last_sequence)
    })
}
