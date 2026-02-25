use openvault_core::operations::filesystem::{commit_filesystem_store, load_filesystem_store};
use openvault_core::vault::runtime::VaultSession;

use crate::errors::Result;
use crate::features::{CommitResult, FilesystemFeature};

pub struct Vault {
    session: VaultSession,
}

impl Vault {
    pub(crate) fn new(session: VaultSession) -> Self {
        Self { session }
    }

    pub fn version(&self) -> u16 {
        self.session.version()
    }

    pub fn filesystem(&mut self) -> FilesystemFeature<'_> {
        FilesystemFeature::new(&mut self.session)
    }

    // pub fn secrets(&mut self) -> SecretsFeature<'_> {
    //     SecretsFeature::new(&mut self.session)
    // }

    pub fn commit_all(&mut self) -> Result<CommitResult> {
        let mut fs_store = load_filesystem_store(&mut self.session)?;
        // let mut secret_store = load_secret_store(&mut self.session)?;

        let filesystem = commit_filesystem_store(&mut self.session, &mut fs_store)?;
        // let secrets = commit_secret_store(&mut self.session, &mut secret_store)?;

        Ok(CommitResult {
            filesystem,
            // secrets,
        })
    }
}
