use std::fs::File;
use std::io::Write;

use openvault_core::errors::Result;
use openvault_core::vault::boot_header::BootHeader;
use openvault_core::vault::crypto::keyring::Keyring;
use openvault_core::vault::features::FeatureType;
use openvault_core::vault::versions::factory::LATEST_VERSION;
use openvault_core::vault::versions::resolve;
use openvault_core::vault::versions::shared::record::{Record, RecordKind};
use openvault_crypto::keys::MasterKey;

fn main() -> Result<()> {
    let handler = resolve(LATEST_VERSION)?;

    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("vault.ov")?;

    let (password, salt) = MasterKey::derive_with_random_salt(b"password")?;

    let keyring = Keyring::new(password);

    let boot_header = BootHeader::new(salt, Some(LATEST_VERSION));

    file.write_all(&boot_header.to_bytes()?)?;

    handler.init_layout(&mut file, &keyring)?;

    let record1 = Record {
        kind: RecordKind::Delta,
        feature_id: FeatureType::Notes,
        payload_version: 1,
        sequence: 1,
        prev_offset: 0,
        key_epoch: 0,
    };

    let first_offset = handler.append_record(&mut file, &record1, b"aabbcc", &keyring)?;

    let record2 = Record {
        kind: RecordKind::Delta,
        feature_id: FeatureType::Filesystem,
        payload_version: 1,
        sequence: 1,
        prev_offset: first_offset,
        key_epoch: 200,
    };

    let _ = handler.append_record(&mut file, &record2, b"", &keyring)?;

    let _ = handler.replay(&mut file, &keyring);

    Ok(())
}
