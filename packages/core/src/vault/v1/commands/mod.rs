pub mod add;
pub mod compact;
pub mod create;
pub mod delete;
pub mod open;
pub mod update;

use std::path::Path;

use crate::errors::Result;
use crate::vault::shared::commands::AddConfig;
use crate::vault::shared::commands::CompactConfig;
use crate::vault::shared::commands::CreateConfig;
use crate::vault::shared::commands::DeleteConfig;
use crate::vault::shared::commands::OpenConfig;
use crate::vault::shared::commands::UpdateConfig;
use crate::vault::shared::commands::VaultCommands;
use crate::vault::v1::schema::vault::Vault;

#[derive(Debug, Default)]
pub struct Commands;

impl VaultCommands for Commands {
    fn create(&self, source: &Path, password: &[u8], config: CreateConfig) -> Result {
        create::run(source, password, Some(config))
    }

    fn open(&self, source: &Path, password: &[u8], config: OpenConfig) -> Result<Vault> {
        open::run(source, password, Some(config))
    }

    fn add(&self, path: &Path, password: &[u8], config: AddConfig) -> Result {
        add::run(path, password, Some(config))
    }

    fn update(&self, path: &Path, password: &[u8], config: UpdateConfig) -> Result {
        update::run(path, password, Some(config))
    }

    fn delete(&self, path: &Path, password: &[u8], config: DeleteConfig) -> Result {
        delete::run(path, password, Some(config))
    }

    fn compact(&self, path: &Path, password: &[u8], config: CompactConfig) -> Result {
        compact::run(path, password, Some(config))
    }
}
