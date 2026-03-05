use crate::errors::Result;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::record::Record;

const CHECKPOINT_INTERVAL: u64 = 200;
// const RECLAIMABLE_BYTES_THRESHOLD: u64 = 16 * 1024 * 1024;

pub fn append_record(session: &mut VaultSession, record: &mut Record) -> Result<u64> {
    let format = session.format();

    let offset =
        session.with_format_context(|file, context| format.append_record(file, record, context))?;

    if should_create_checkpoint(session)? {
        create_checkpoint(session)?;
    }

    Ok(offset)
}

pub fn create_checkpoint(_session: &mut VaultSession) -> Result<u64> {
    // @todo-now continue with this..
    todo!();

    // let format = session.format();

    // session.with_format_context(|file, context| {
    //     let subheader = format.read_subheader(file, context)?;
    //     let mut checkpoint = Checkpoint {
    //         features: Vec::new(),
    //         sequence: subheader.last_sequence,
    //     };

    //     format.write_checkpoint(file, &mut checkpoint, context)
    // })
}

fn should_create_checkpoint(session: &mut VaultSession) -> Result<bool> {
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
