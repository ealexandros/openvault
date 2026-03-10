use openvault_core::features::filesystem::FilesystemStore;
use openvault_core::features::messages::MessagesStore;
use openvault_core::operations::{compact, history, replay};
use openvault_core::repositories::{FeatureRepository, FilesystemRepository, MessagesRepository};
use openvault_core::vault::runtime::VaultSession;
use openvault_core::vault::versions::shared::checkpoint::Checkpoint;
use zeroize::Zeroize;

use crate::errors::Result;
use crate::features::filesystem::FilesystemService;
use crate::features::messages::MessagesService;

#[derive(Zeroize)]
pub struct Vault {
    session: VaultSession,
    filesystem: FilesystemStore,
    messages: MessagesStore,
}

impl Vault {
    pub(crate) fn new(mut session: VaultSession) -> Result<Self> {
        let replay = replay::replay_since_checkpoint(&mut session)?;

        let filesystem = FilesystemRepository::restore_from_replay(&replay)?;
        let messages = MessagesRepository::restore_from_replay(&replay)?;

        Ok(Self {
            session,
            filesystem,
            messages,
        })
    }

    pub fn version(&self) -> u16 {
        self.session.version()
    }

    pub fn commit(&mut self) -> Result {
        FilesystemRepository::commit(&mut self.session, &mut self.filesystem)?;
        MessagesRepository::commit(&mut self.session, &mut self.messages)?;

        if !history::should_create_checkpoint(&mut self.session)? {
            return Ok(());
        }

        self.commit_checkpoint()
    }

    fn commit_checkpoint(&mut self) -> Result {
        let checkpoint_features = vec![
            FilesystemRepository::create_checkpoint(&mut self.filesystem)?,
            MessagesRepository::create_checkpoint(&mut self.messages)?,
        ];

        let mut checkpoint = Checkpoint::new(checkpoint_features);
        history::create_checkpoint(&mut self.session, &mut checkpoint)?;

        Ok(())
    }

    pub fn compact(&mut self) -> Result {
        self.commit()?;

        // @todo-now return the new features..

        compact::compact_vault(&mut self.session)?;

        self.filesystem = FilesystemRepository::load(&mut self.session)?;
        self.messages = MessagesRepository::load(&mut self.session)?;

        Ok(())
    }

    #[inline]
    pub fn filesystem(&mut self) -> FilesystemService<'_> {
        FilesystemService::new(&mut self.session, &mut self.filesystem)
    }

    #[inline]
    pub fn messages(&mut self) -> MessagesService<'_> {
        MessagesService::new(&mut self.session, &mut self.messages)
    }
}
