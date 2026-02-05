use crate::errors::{Error, Result};
use std::fs::File;
use std::path::Path;

pub trait PathExt {
    fn relative_to(&self, root: &Path) -> Result<String>;
    fn ensure_file_exists(&self) -> Result;
    fn create_new_file(&self) -> Result<File>;
    fn remove_file_if_exists(&self) -> Result;
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

    fn ensure_file_exists(&self) -> Result {
        if !self.exists() {
            return Err(Error::file_not_found());
        }
        if !self.is_file() {
            return Err(Error::file_not_a_file());
        }
        Ok(())
    }

    fn create_new_file(&self) -> Result<File> {
        if self.exists() {
            return Err(Error::file_exists());
        }
        File::create(self).map_err(Error::Io)
    }

    fn remove_file_if_exists(&self) -> Result {
        if self.exists() {
            std::fs::remove_file(self).map_err(Error::Io)?;
        }
        Ok(())
    }
}
