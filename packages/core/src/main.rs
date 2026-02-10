use openvault_core::crypto::compression::factory::CompressionAlgorithm;
use openvault_core::crypto::encryption::factory::EncryptionAlgorithm;
use openvault_core::vault::shared::commands::{CreateConfig, OpenConfig};
use openvault_core::vault::versions::VaultVersion;
use std::path::Path;

// @todo-now this needs to be remove after testing

fn main() {
    let path = Path::new("./temp/myfiles");
    let password = b"password";

    let create_config = CreateConfig {
        compression: CompressionAlgorithm::Zstd,
        cipher: EncryptionAlgorithm::XChaCha20Poly1305,
        output_path: "./temp".to_string(),
        filename: "vault.ov".to_string(),
        overwrite_existing: true,
    };

    let open_config = OpenConfig {};

    let vault = VaultVersion::V1;
    let commands = vault.commands();

    commands.create(path, password, create_config).unwrap();

    let output = commands
        .open(
            path.parent().unwrap().join("vault.ov").as_path(),
            password,
            open_config,
        )
        .unwrap();

    println!("{:#?}", output);
}
