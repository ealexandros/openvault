use openvault_core::features::filesystem::ROOT_FOLDER_ID;
use openvault_sdk::{
    CompressionAlgorithm, CreateConfig, EncryptionAlgorithm, FileMetadata, FolderMetadata,
    create_and_open_vault,
};

fn main() {
    let path = "./temp/vault.ov";
    let password = "password";

    let config = CreateConfig::default()
        .with_compression(CompressionAlgorithm::Zstd)
        .with_encryption(EncryptionAlgorithm::XChaCha20Poly1305)
        .with_overwrite(true);

    let mut vault = create_and_open_vault(path, password, config).unwrap();

    let mut fs = vault.filesystem();

    let folder = FolderMetadata::new(Some(ROOT_FOLDER_ID), "my_directory");
    let folder_id = fs.add_directory(folder).unwrap();

    let blob_ref = fs.put_blob_bytes(b"hey").unwrap();
    let mut file = FileMetadata::new(folder_id, "myfile.txt");
    file.mime_type = Some("text/plain".to_string());
    file.blob = Some(blob_ref);

    let file_id = fs.add_file(file).unwrap();

    fs.commit().unwrap();

    println!("{:#?}", fs.directories());
    println!("{:#?}", fs.files());

    println!(
        "{:#?}",
        String::from_utf8(fs.get_file(&file_id).unwrap().unwrap()).unwrap()
    );
}
