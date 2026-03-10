pub mod filesystem;
pub mod messages;

use openvault_core::repositories::CommitOutcome;
use openvault_core::vault::runtime::VaultSession;
use openvault_core::vault::versions::shared::checkpoint::CheckpointFeature;

use crate::errors::Result;

pub(crate) trait FeatureRuntime {
    fn commit(&mut self, session: &mut VaultSession) -> Result<CommitOutcome>;
    fn create_checkpoint(&self) -> Result<CheckpointFeature>;
}
