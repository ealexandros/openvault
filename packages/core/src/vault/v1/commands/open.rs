use crate::errors::Result;
use crate::vault::shared::commands::OpenConfig;
use crate::vault::v1::structure::Vault;
use std::path::Path;

pub fn run(_source_path: &Path, _password: &[u8], _config: Option<OpenConfig>) -> Result<Vault> {
    todo!()
}
