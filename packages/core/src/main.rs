use openvault_core::{
    crypto::{
        compression::factory::CompressionAlgorithm, encryption::factory::EncryptionAlgorithm,
    },
    vault::{shared::commands::CreateConfig, versions::VaultVersion},
};
use std::{path::Path, str::FromStr};

// @todo-now this needs to be remove after testing

fn main() {
    let path = Path::new("./temp/myfiles");
    let password = b"password";

    let config = CreateConfig {
        compression: CompressionAlgorithm::Zstd,
        cipher: EncryptionAlgorithm::XChaCha20Poly1305,
        output_path: "./temp".to_string(),
        filename: "vault.ov".to_string(),
        overwrite_existing: true,
    };

    VaultVersion::from_str("v1")
        .unwrap()
        .commands()
        .create(path, password, config)
        .unwrap();
}
