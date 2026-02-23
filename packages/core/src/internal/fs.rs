use std::fs::File;
use std::path::Path;

use crate::errors::{Error, Result};

pub fn open_with_read_write(path: &Path) -> Result<File> {
    if !path.exists() {
        return Err(Error::file_not_exists());
    }

    File::options()
        .read(true)
        .write(true)
        .open(path)
        .map_err(Error::Io)
}

pub fn create_new_file(path: &Path) -> Result<File> {
    File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(Error::Io)
}

pub fn remove_if_exists(path: &Path) -> Result {
    if path.exists() {
        std::fs::remove_file(path).map_err(Error::Io)?;
    }
    Ok(())
}
