use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to unlock vault. Verify password and selected algorithms")]
    UnlockFailed,

    #[error(transparent)]
    Core(openvault_core::errors::Error),
}

impl From<openvault_core::errors::Error> for Error {
    fn from(value: openvault_core::errors::Error) -> Self {
        match value {
            openvault_core::errors::Error::UnlockFailed => Self::UnlockFailed,
            other => Self::Core(other),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;
