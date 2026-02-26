use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] openvault_core::errors::Error),

    #[error("Filesystem error: {0}")]
    Filesystem(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;
