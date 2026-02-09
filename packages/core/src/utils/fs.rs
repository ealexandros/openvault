use crate::errors::{Error, Result};
use std::fs::File;
use std::path::Path;

pub fn ensure_file_exists(path: &Path) -> Result {
    if !path.exists() {
        return Err(Error::file_not_found());
    }
    if !path.is_file() {
        return Err(Error::file_not_a_file());
    }
    Ok(())
}

pub fn create_new_file(path: &Path) -> Result<File> {
    if path.exists() {
        return Err(Error::file_exists());
    }
    File::create(path).map_err(Error::Io)
}

pub fn remove_file_if_exists(path: &Path) -> Result {
    if path.exists() {
        std::fs::remove_file(path).map_err(Error::Io)?;
    }
    Ok(())
}
