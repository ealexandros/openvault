use crate::errors::Result;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::replay::ReplayState;

pub fn replay_since_checkpoint(session: &mut VaultSession) -> Result<ReplayState> {
    let format = session.format();
    session.with_format_context(|file, context| format.replay(file, context))
}
