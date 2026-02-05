use crate::errors::{Error, Result};
use std::fs::File;
use std::path::Path;

pub trait PathExt {
    fn relative_to(&self, root: &Path) -> Result<String>;
}

impl PathExt for Path {
    fn relative_to(&self, root: &Path) -> Result<String> {
        Ok(self
            .strip_prefix(root)
            .map_err(|_| Error::InvalidPath)?
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/"))
    }
}

pub fn ensure_file_exists(path: &Path) -> Result<()> {
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
