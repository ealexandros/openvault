use std::path::Path;

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

    let _ = fs.add_folder(Uuid::nil(), "test1".to_string()).unwrap();
    let _ = fs.add_folder(Uuid::nil(), "test2".to_string()).unwrap();
    let folder_id = fs.add_folder(Uuid::nil(), "test3".to_string()).unwrap();
    let folder_id2 = fs.add_folder(folder_id, "test4".to_string()).unwrap();

    fs.delete_folder(folder_id2).unwrap();

    let file_id = fs
        .add_file(Uuid::nil(), Path::new("./src/main.rs"))
        .unwrap();

    fs.commit().unwrap();

    println!(
        "{}",
        String::from_utf8(fs.get_file_content(file_id).unwrap().unwrap()).unwrap()
    );

    // println!("{:#?}", fs.browse(&Uuid::nil()).unwrap());
}
