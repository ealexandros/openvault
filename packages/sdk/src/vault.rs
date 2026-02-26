use openvault_core::features::filesystem::FilesystemStore;
use openvault_core::operations::filesystem::FilesystemOps;
use openvault_core::vault::runtime::VaultSession;

use crate::errors::Result;
use crate::features::{CommitResult, FilesystemFeature};

pub struct Vault {
    session: VaultSession,
    filesystem_store: FilesystemStore,
}

impl Vault {
    pub(crate) fn new(mut session: VaultSession) -> Result<Self> {
        let filesystem_store = FilesystemOps::load(&mut session)?;

        Ok(Self {
            session,
            filesystem_store,
        })
    }

    pub fn version(&self) -> u16 {
        self.session.version()
    }

    pub fn filesystem(&mut self) -> FilesystemFeature<'_> {
        FilesystemFeature::new(&mut self.session, &mut self.filesystem_store)
    }

    pub fn commit_all(&mut self) -> Result<CommitResult> {
        let filesystem = self.filesystem().commit()?;

        Ok(CommitResult { filesystem })
    }
}
