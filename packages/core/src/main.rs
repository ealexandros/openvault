use std::fs::File;
use std::io::Write;

use openvault_core::errors::Result;
use openvault_core::vault::boot_header::BootHeader;
use openvault_core::vault::crypto::keyring::Keyring;
use openvault_core::vault::features::FeatureType;
use openvault_core::vault::versions::factory::LATEST_VERSION;
use openvault_core::vault::versions::get_handler;
use openvault_core::vault::versions::shared::record::{Record, RecordKind};
use openvault_crypto::keys::MasterKey;

fn main() -> Result<()> {
    let handler = get_handler(LATEST_VERSION)?;

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

    let record = Record {
        kind: RecordKind::Delta,
        feature_id: FeatureType::Filesystem,
        payload_version: 1,
        sequence: 1,
        prev_offset: 0,
        payload_size: 0,
        key_epoch: 0,
    };

    let first_offset = handler.append_record(&mut file, &record, b"", &keyring)?;
    handler.append_record(&mut file, &record, b"", &keyring)?;

    let (record, payload) = handler.read_record(&mut file, first_offset, &keyring)?;

    println!("Record: {:#?}", record);
    println!("Payload: {:#?}", payload);

    Ok(())
}
