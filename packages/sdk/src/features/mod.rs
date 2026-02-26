mod filesystem;
// mod secrets;

pub use filesystem::FilesystemFeature;
// pub use secrets::SecretsFeature;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CommitResult {
    pub filesystem: bool,
}

impl CommitResult {
    pub fn any(self) -> bool {
        self.filesystem
    }
}
