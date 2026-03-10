use openvault_core::features::messages::MessagesStore;
use openvault_core::repositories::{CommitOutcome, FeatureRepository, MessagesRepository};
use openvault_core::vault::runtime::VaultSession;
use openvault_core::vault::versions::shared::checkpoint::CheckpointFeature;
use openvault_core::vault::versions::shared::replay::ReplayState;
use zeroize::Zeroize;

use crate::errors::Result;
use crate::features::FeatureRuntime;
use crate::features::messages::MessagesService;

#[derive(Zeroize)]
pub(crate) struct MessagesRuntime {
    store: MessagesStore,
}

impl MessagesRuntime {
    pub fn load(session: &mut VaultSession) -> Result<Self> {
        let store = MessagesRepository::load(session)?;

        Ok(Self { store })
    }

    pub fn restore_from_replay(replay: &ReplayState) -> Result<Self> {
        let store = MessagesRepository::restore_from_replay(replay)?;

        Ok(Self { store })
    }

    pub fn service<'a>(&'a mut self, session: &'a mut VaultSession) -> MessagesService<'a> {
        MessagesService::new(session, &mut self.store)
    }
}

impl FeatureRuntime for MessagesRuntime {
    fn commit(&mut self, session: &mut VaultSession) -> Result<CommitOutcome> {
        MessagesRepository::commit(session, &mut self.store).map_err(From::from)
    }

    fn create_checkpoint(&self) -> Result<CheckpointFeature> {
        MessagesRepository::create_checkpoint(&self.store).map_err(From::from)
    }
}
