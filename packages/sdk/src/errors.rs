use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(openvault_core::errors::Error),

    #[error("Unable to unlock vault. Verify password and selected algorithms")]
    UnlockFailed,

    #[error("Filesystem error: {0}")]
    Filesystem(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl From<openvault_core::errors::Error> for Error {
    fn from(error: openvault_core::errors::Error) -> Self {
        match error {
            openvault_core::errors::Error::UnlockFailed => Self::UnlockFailed,
            other => Self::Core(other),
        }
    }
}
