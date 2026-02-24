use openvault_core::features::filesystem::FilesystemStore;
use openvault_core::features::secrets::SecretStore;

#[derive(Debug, Clone)]
pub struct VaultStores {
    pub filesystem: FilesystemStore,
    pub secrets: SecretStore,
}

impl VaultStores {
    pub fn new(filesystem: FilesystemStore, secrets: SecretStore) -> Self {
        Self {
            filesystem,
            secrets,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StoresCommitResult {
    pub filesystem: bool,
    pub secrets: bool,
}

impl StoresCommitResult {
    pub fn any(self) -> bool {
        self.filesystem || self.secrets
    }
}
