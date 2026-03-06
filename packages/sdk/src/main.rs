use openvault_sdk::{
    CompressionAlgorithm, CreateConfig, EncryptionAlgorithm, create_and_open_vault,
};
use uuid::Uuid;

fn main() {
    let path = "../../temp/example/Vault.ov";
    let password = "password";

    let config = CreateConfig::default()
        .with_compression(CompressionAlgorithm::Zstd)
        .with_encryption(EncryptionAlgorithm::XChaCha20Poly1305)
        .with_overwrite(true);

    let mut vault = create_and_open_vault(path, password, config).unwrap();

    let mut fs = vault.filesystem();

    let _ = fs.add_folder(Uuid::nil(), "test1".to_string()).unwrap();
    let _ = fs.add_folder(Uuid::nil(), "test2".to_string()).unwrap();
    let folder_id = fs.add_folder(Uuid::nil(), "test3".to_string()).unwrap();
    let _ = fs.add_folder(folder_id, "test4".to_string()).unwrap();

    vault.commit().unwrap();
}
