mod filesystem;

pub use filesystem::FilesystemFeature;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CommitResult {
    pub filesystem: bool,
}

impl CommitResult {
    pub fn any(self) -> bool {
        self.filesystem
    }
}
