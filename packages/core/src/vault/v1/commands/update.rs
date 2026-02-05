use crate::errors::Result;
use crate::vault::shared::commands::UpdateConfig;
use crate::vault::v1::structure::Vault;
use std::path::Path;

pub fn run(_path: &Path, _password: &[u8], _config: Option<UpdateConfig>) -> Result<Vault> {
    todo!()
}
