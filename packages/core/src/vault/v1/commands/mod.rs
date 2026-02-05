pub mod create;
pub mod delete;
pub mod open;
pub mod update;

use std::path::Path;

use crate::errors::{Error, Result};
use crate::vault::shared::commands::{
    CreateConfig, DeleteConfig, OpenConfig, UpdateConfig, VaultCommands,
};

#[derive(Debug, Default)]
pub struct Commands;

impl VaultCommands for Commands {
    fn create(&self, source: &Path, password: &[u8], config: CreateConfig) -> Result<()> {
        create::run(source, password, Some(config))
    }

    fn open(&self, _path: &Path, _password: &[u8], _config: OpenConfig) -> Result<()> {
        Err(Error::UnsupportedCommand("open (v1)".to_string()))
    }

    fn update(&self, _path: &Path, _password: &[u8], _config: UpdateConfig) -> Result<()> {
        Err(Error::UnsupportedCommand("update (v1)".to_string()))
    }

    fn delete(&self, _path: &Path, _password: &[u8], _config: DeleteConfig) -> Result<()> {
        Err(Error::UnsupportedCommand("delete (v1)".to_string()))
    }
}
