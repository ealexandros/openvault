use openvault_sdk::open_vault;
use uuid::Uuid;

fn main() {
    let path = "../../temp/example/Vault.ov";
    let password = "password";

    // let config = CreateConfig::default()
    //     .with_compression(CompressionAlgorithm::Zstd)
    //     .with_encryption(EncryptionAlgorithm::XChaCha20Poly1305)
    //     .with_overwrite(true);

    let mut vault = open_vault(path, password).unwrap();

    let mut fs = vault.filesystem();

    // let _ = fs.add_folder(Uuid::nil(), "test1".to_string()).unwrap();
    // let _ = fs.add_folder(Uuid::nil(), "test2".to_string()).unwrap();
    // let folder_id = fs.add_folder(Uuid::nil(), "test3".to_string()).unwrap();
    // let folder_id2 = fs.add_folder(folder_id, "test4".to_string()).unwrap();

    // fs.delete_folder(folder_id2).unwrap();

    // let file_id = fs
    //     .add_file(Uuid::nil(), Path::new("./src/main.rs"))
    //     .unwrap();

    // fs.commit().unwrap();

    let (folders, files) = fs.browse(&Uuid::nil()).unwrap();

    println!("{:#?}", folders);
    println!("{:#?}", files);

    println!(
        "{}",
        String::from_utf8(fs.get_file_content(files[0].id).unwrap().unwrap()).unwrap()
    );
}
