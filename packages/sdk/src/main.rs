use openvault_sdk::{
    CompressionAlgorithm, CreateConfig, EncryptionAlgorithm, create_and_open_vault,
};
use uuid::Uuid;

fn main() {
    let path = "./temp/vault.ov";
    let password = "password";

    let config = CreateConfig::default()
        .with_compression(CompressionAlgorithm::Zstd)
        .with_encryption(EncryptionAlgorithm::XChaCha20Poly1305)
        .with_overwrite(true);

    let mut vault = create_and_open_vault(path, password, config).unwrap();

    let mut fs = vault.filesystem();

    fs.add_directory(openvault_sdk::FolderMetadata::new(
        Uuid::new_v4(),
        Some(Uuid::nil()),
        "my_directory",
    ))
    .unwrap();

    fs.commit().unwrap();

    println!("{:#?}", fs.directories());
}
