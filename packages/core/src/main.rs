use std::path::Path;

use openvault_core::errors::Result;
use openvault_core::operations::config::CreateConfig;
use openvault_core::operations::vault::create_and_open_vault;

fn main() -> Result {
    let password = b"password";
    let path = Path::new("./temp/vault.ov");

    let create_config = CreateConfig::new().with_overwrite(true);

    let session = create_and_open_vault(&path, password, create_config)?;

    println!("Vault created at: {}", path.display());
    println!("Vault version: {}", session.engine().version());

    Ok(())
}
