use openvault_crypto::{compression::CompressionAlgorithm, encryption::EncryptionAlgorithm};
use openvault_sdk::{operations::create::CreateConfig, vault::operations::create::create_vault};

fn main() {
    let vault_path = "./temp".to_string();
    let password = "password".to_string();
    let output_path = "./temp".to_string();
    let filename = "test.ov".to_string();

    let config = CreateConfig {
        compression: CompressionAlgorithm::Zstd,
        cipher: EncryptionAlgorithm::XChaCha20Poly1305,
        output_path,
        filename,
        overwrite_existing: true,
    };

    let _ = create_vault(vault_path, password, config).unwrap();
}
