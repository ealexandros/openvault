use std::io::Cursor;
use std::path::Path;

use openvault_core::errors::Result;
use openvault_core::features::filesystem::scan_directory;
use openvault_core::operations::blob::{get_blob, put_blob};
use openvault_core::operations::config::CreateConfig;
use openvault_core::operations::vault::create_and_open_vault;

fn main() -> Result {
    let password = b"password";
    let path = Path::new("./temp/vault.ov");

    let create_config = CreateConfig::new().with_overwrite(true);

    let mut session = create_and_open_vault(&path, password, create_config)?;

    let mut blob_cursor = Cursor::new(b"this is my simple test");

    let blob_ref = put_blob(&mut session, &mut blob_cursor)?;
    let blob = get_blob(&mut session, &blob_ref)?;

    println!("Vault created at: {}", path.display());
    println!("Vault version: {}", session.format().version());
    println!("Blob: {}", String::from_utf8_lossy(&blob));

    let (files, folders) = scan_directory(Path::new("./temp"))?;

    println!("Files: {:#?}", files);
    println!("Folders: {:#?}", folders);

    Ok(())
}
