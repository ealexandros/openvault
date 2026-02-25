// @todo-now uncomment this..

// use std::fs;
// use std::path::PathBuf;
// use std::time::{SystemTime, UNIX_EPOCH};

// use openvault_sdk::{
//     CreateVaultOptions, EncryptedField, Error, FeatureFacade, LoginEntry, VaultClient,
// };

// fn temp_vault_path(name: &str) -> PathBuf {
//     let timestamp = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("clock drift")
//         .as_nanos();
//     let dir = std::env::temp_dir().join(format!("openvault_sdk_{name}_{timestamp}"));
//     fs::create_dir_all(&dir).expect("create temp directory");
//     dir.join("vault.ov")
// }

// #[test]
// fn sdk_open_wrong_password_returns_unlock_failed() {
//     let path = temp_vault_path("unlock_failed");
//     let root = path.parent().expect("temp root").to_path_buf();

//     let client = VaultClient::new();
//     client
//         .create_with(&path, b"password", CreateVaultOptions::default())
//         .expect("create vault");

//     let result = client.open(&path, b"wrong-password");
//     assert!(matches!(result, Err(Error::UnlockFailed)));

//     fs::remove_dir_all(root).expect("cleanup");
// }

// #[test]
// fn sdk_filesystem_store_stages_and_commits_blob_backed_file() {
//     let path = temp_vault_path("filesystem_feature");
//     let root = path.parent().expect("temp root").to_path_buf();
//     let source = root.join("readme.md");
//     fs::write(&source, b"sdk-blob").expect("write source file");

//     let client = VaultClient::new();
//     client
//         .create_with(&path, b"password", CreateVaultOptions::default())
//         .expect("create vault");

//     {
//         let mut handle = client.open(&path, b"password").expect("open vault");
//         let mut filesystem = handle.filesystem();
//         let file_id = filesystem
//             .add_file_from_path(&source)
//             .expect("stage file from path");

//         let state = filesystem.get_state().expect("filesystem state");
//         assert_eq!(state.files().len(), 1);
//         assert_eq!(state.files()[0].id, file_id);

//         let file_blob = filesystem
//             .get_file(&file_id)
//             .expect("get staged file")
//             .expect("blob available");
//         assert_eq!(file_blob, b"sdk-blob");

//         let commit = handle.commit_all().expect("commit all stores");
//         assert!(commit.filesystem);
//         assert!(!commit.secrets);
//     }

//     {
//         let mut reopened = client.open(&path, b"password").expect("reopen vault");
//         let mut filesystem = reopened.filesystem();
//         let files = filesystem.files().expect("list files");

//         assert_eq!(files.len(), 1);
//         assert_eq!(files[0].name, "readme.md");

//         let file_blob = filesystem
//             .get_file(&files[0].id)
//             .expect("get persisted file")
//             .expect("blob available");
//         assert_eq!(file_blob, b"sdk-blob");
//     }

//     fs::remove_dir_all(root).expect("cleanup");
// }

// #[test]
// fn sdk_secret_store_persists_changes() {
//     let path = temp_vault_path("secrets_feature");
//     let root = path.parent().expect("temp root").to_path_buf();

//     let client = VaultClient::new();
//     client
//         .create_with(&path, b"password", CreateVaultOptions::default())
//         .expect("create vault");

//     {
//         let mut handle = client.open(&path, b"password").expect("open vault");
//         let mut secrets = handle.secrets();

//         let entry = LoginEntry::new(
//             Some("/accounts".to_string()),
//             "mail".to_string(),
//             "alex".to_string(),
//             EncryptedField::new(vec![1, 2, 3, 4]),
//             Some("https://mail.example".to_string()),
//             None,
//             None,
//         )
//         .expect("build secret");

//         let entry_id = secrets.insert(entry).expect("insert secret");
//         let staged = secrets.get_entry(&entry_id).expect("lookup staged secret");
//         assert!(staged.is_some());

//         let wrote = secrets.commit().expect("commit secret store");
//         assert!(wrote);
//     }

//     {
//         let mut reopened = client.open(&path, b"password").expect("reopen vault");
//         let mut secrets = reopened.secrets();
//         let entries = secrets.list_all().expect("list secrets");
//         assert_eq!(entries.len(), 1);
//         assert_eq!(entries[0].name, "mail");
//     }

//     fs::remove_dir_all(root).expect("cleanup");
// }

// #[test]
// fn sdk_commit_all_commits_all_loaded_feature_states() {
//     let path = temp_vault_path("centralized_stores");
//     let root = path.parent().expect("temp root").to_path_buf();
//     let source = root.join("todo.txt");
//     fs::write(&source, b"todo-content").expect("write source file");

//     let client = VaultClient::new();
//     client
//         .create_with(&path, b"password", CreateVaultOptions::default())
//         .expect("create vault");

//     {
//         let mut handle = client.open(&path, b"password").expect("open vault");

//         {
//             let mut filesystem = handle.filesystem();
//             filesystem
//                 .add_file_from_path(&source)
//                 .expect("stage file in filesystem");
//         }

//         {
//             let mut secrets = handle.secrets();
//             let entry = LoginEntry::new(
//                 Some("/work".to_string()),
//                 "tracker".to_string(),
//                 "alex".to_string(),
//                 EncryptedField::new(vec![9, 9, 9]),
//                 None,
//                 None,
//                 None,
//             )
//             .expect("build secret");

//             secrets.insert(entry).expect("stage secret");
//         }

//         let state = handle.get_state().expect("get in-memory state");
//         assert_eq!(state.filesystem.files().len(), 1);
//         assert_eq!(state.secrets.list_all().len(), 1);

//         let commit = handle.commit_all().expect("commit all");
//         assert!(commit.filesystem);
//         assert!(commit.secrets);
//         assert!(commit.any());
//     }

//     {
//         let mut reopened = client.open(&path, b"password").expect("reopen vault");
//         let state = reopened.get_state().expect("load state");
//         assert_eq!(state.filesystem.files().len(), 1);
//         assert_eq!(state.filesystem.files()[0].name, "todo.txt");
//         assert_eq!(state.secrets.list_all().len(), 1);
//         assert_eq!(state.secrets.list_all()[0].name, "tracker");
//     }

//     fs::remove_dir_all(root).expect("cleanup");
// }
