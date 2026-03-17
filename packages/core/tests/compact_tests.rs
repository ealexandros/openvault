use std::io::Cursor;

use uuid::Uuid;

use openvault_core::features::filesystem::FILESYSTEM_ROOT_FOLDER_ID;
use openvault_core::operations::blob::{read_blob, write_blob};
use openvault_core::operations::compact::compact_vault;
use openvault_core::operations::vault::create_and_open_vault;
use openvault_core::repositories::{FeatureRepository, FilesystemRepository};

fn temp_vault_path() -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("openvault_compact_test_{}.ov", Uuid::new_v4()));
    path
}

#[test]
fn compaction_rewrites_blob_refs_and_preserves_latest_state() {
    let vault_path = temp_vault_path();

    let mut session = create_and_open_vault(&vault_path, b"password", Default::default())
        .expect("create and open vault");

    let mut store = FilesystemRepository::load(&mut session).expect("load filesystem");

    let large_payload = vec![77u8; 2 * 1024 * 1024];
    let mut large_reader = Cursor::new(large_payload);
    let stale_blob = write_blob(&mut session, &mut large_reader).expect("write large blob");

    let stale_file = store
        .add_file(
            FILESYSTEM_ROOT_FOLDER_ID,
            "stale.bin".to_string(),
            "bin".to_string(),
            stale_blob,
        )
        .expect("add stale file");
    FilesystemRepository::commit(&mut session, &mut store).expect("commit stale file");

    store.remove_file(stale_file).expect("remove stale file");
    FilesystemRepository::commit(&mut session, &mut store).expect("commit stale delete");

    let live_payload = b"live-data".to_vec();
    let mut live_reader = Cursor::new(live_payload.clone());
    let live_blob = write_blob(&mut session, &mut live_reader).expect("write live blob");

    let live_file = store
        .add_file(
            FILESYSTEM_ROOT_FOLDER_ID,
            "live.txt".to_string(),
            "txt".to_string(),
            live_blob,
        )
        .expect("add live file");
    FilesystemRepository::commit(&mut session, &mut store).expect("commit live file");

    let before_size = session.file().metadata().expect("before metadata").len();

    compact_vault(&mut session).expect("compact vault");

    let after_size = session.file().metadata().expect("after metadata").len();
    assert!(
        after_size < before_size,
        "expected compacted vault to shrink: before={before_size}, after={after_size}"
    );

    let reloaded = FilesystemRepository::load(&mut session).expect("reload filesystem");
    let live_file_metadata = reloaded.file(&live_file).expect("live file exists");

    let restored = read_blob(&mut session, &live_file_metadata.blob).expect("read live blob");
    assert_eq!(restored, live_payload);

    std::fs::remove_file(vault_path).expect("cleanup vault");
}
