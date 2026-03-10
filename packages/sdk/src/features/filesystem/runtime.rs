use openvault_core::features::filesystem::FilesystemStore;
use openvault_core::repositories::{CommitOutcome, FeatureRepository, FilesystemRepository};
use openvault_core::vault::runtime::VaultSession;
use openvault_core::vault::versions::shared::checkpoint::CheckpointFeature;
use openvault_core::vault::versions::shared::replay::ReplayState;
use zeroize::Zeroize;

use crate::errors::Result;
use crate::features::FeatureRuntime;
use crate::features::filesystem::FilesystemService;

#[derive(Zeroize)]
pub(crate) struct FilesystemRuntime {
    store: FilesystemStore,
}

impl FilesystemRuntime {
    pub fn load(session: &mut VaultSession) -> Result<Self> {
        let store = FilesystemRepository::load(session)?;

        Ok(Self { store })
    }

    pub fn restore_from_replay(replay: &ReplayState) -> Result<Self> {
        let store = FilesystemRepository::restore_from_replay(replay)?;

        Ok(Self { store })
    }

    pub fn service<'a>(&'a mut self, session: &'a mut VaultSession) -> FilesystemService<'a> {
        FilesystemService::new(session, &mut self.store)
    }
}

impl FeatureRuntime for FilesystemRuntime {
    fn commit(&mut self, session: &mut VaultSession) -> Result<CommitOutcome> {
        FilesystemRepository::commit(session, &mut self.store).map_err(From::from)
    }

    fn create_checkpoint(&self) -> Result<CheckpointFeature> {
        FilesystemRepository::create_checkpoint(&self.store).map_err(From::from)
    }
}
