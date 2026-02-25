mod filesystem;
mod secrets;

pub use filesystem::FilesystemStore;
pub use secrets::SecretsStore;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CommitResult {
    pub filesystem: bool,
    pub secrets: bool,
}

impl CommitResult {
    pub fn any(self) -> bool {
        self.filesystem || self.secrets
    }
}
