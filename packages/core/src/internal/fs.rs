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

pub fn resolve_path(path: &Path, filename: &str, extension: &str) -> std::path::PathBuf {
    let mut resolved = path.to_path_buf();

    if !filename.is_empty() {
        if path.is_dir() {
            resolved.push(filename);
        } else {
            resolved.set_file_name(filename);
        }
    }

    if resolved.extension().map_or(true, |ext| ext != extension) {
        resolved.set_extension(extension);
    }

    resolved
}
