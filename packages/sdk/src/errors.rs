use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] openvault_core::errors::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Invalid path")]
    InvalidPath,

    #[error("Item already exists: {0}")]
    ItemAlreadyExists(String),
}

pub type Result<T = ()> = std::result::Result<T, Error>;
