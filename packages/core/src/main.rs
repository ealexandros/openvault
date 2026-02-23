use std::path::Path;

use openvault_core::errors::Result;
use openvault_core::operations::vault::create_and_open_vault;

fn main() -> Result {
    let password = b"password";
    let path = Path::new("./temp/vault.ov");

    let session = create_and_open_vault(&path, password)?;

    println!("Vault created at: {}", path.display());
    println!("Vault version: {}", session.engine().version());

    Ok(())
}
