use std::path::Path;

use crate::errors::Result;
use crate::vault::shared::commands::DeleteConfig;

pub fn run(_vault_path: &Path, _password: &[u8], _config: Option<DeleteConfig>) -> Result {
    todo!()
}
