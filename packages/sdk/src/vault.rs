use openvault_core::operations::{compact, history};
use openvault_core::vault::runtime::VaultSession;
use openvault_core::vault::versions::shared::checkpoint::Checkpoint;
use zeroize::Zeroize;

use crate::errors::Result;
use crate::features::FeatureRuntime;
use crate::features::filesystem::{FilesystemRuntime, FilesystemService};

#[derive(Zeroize)]
pub struct Vault {
    session: VaultSession,
    filesystem: FilesystemRuntime,
}

impl Vault {
    pub(crate) fn new(mut session: VaultSession) -> Result<Self> {
        let filesystem = FilesystemRuntime::load(&mut session)?;

        Ok(Self {
            session,
            filesystem,
        })
    }

    pub fn version(&self) -> u16 {
        self.session.version()
    }

    #[inline]
    pub fn filesystem(&mut self) -> FilesystemService<'_> {
        self.filesystem.service(&mut self.session)
    }

    pub fn commit(&mut self) -> Result {
        self.filesystem.commit(&mut self.session)?;

        if !history::should_create_checkpoint(&mut self.session)? {
            return Ok(());
        }

        self.commit_checkpoint()
    }

    pub fn compact(&mut self) -> Result {
        self.commit()?;
        compact::compact_vault(&mut self.session)?;

        self.filesystem = FilesystemRuntime::load(&mut self.session)?;

        Ok(())
    }

    fn commit_checkpoint(&mut self) -> Result {
        let checkpoint_features = vec![self.filesystem.create_checkpoint()?];
        let mut checkpoint = Checkpoint::new(checkpoint_features);

        history::create_checkpoint(&mut self.session, &mut checkpoint)?;

        Ok(())
    }
}
