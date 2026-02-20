use std::fs::File;
use std::io::Write;

use openvault_core::errors::Result;
use openvault_core::vault::boot_header::BootHeader;
use openvault_core::vault::crypto::keyring::Keyring;
use openvault_core::vault::features::FeatureType;
use openvault_core::vault::versions;
use openvault_core::vault::versions::factory::LATEST_VERSION;
use openvault_core::vault::versions::shared::record::RecordHeader;
use openvault_crypto::keys::MasterKey;

fn main() -> Result<()> {
    let handler = versions::resolve_latest()?;

    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("./temp/vault.ov")?;

    let (password, salt) = MasterKey::derive_with_random_salt(b"password")?;

    let keyring = Keyring::new(password);

    let boot_header = BootHeader::new(salt, Some(LATEST_VERSION));

    file.write_all(&boot_header.to_bytes()?)?;

    handler.init_layout(&mut file, &keyring)?;

    let record1 = RecordHeader::new(FeatureType::Notes, 1);

    handler.append_record(&mut file, &record1, b"aabbcc", &keyring)?;

    let record2 = RecordHeader::new(FeatureType::Filesystem, 1);

    let _ = handler.append_record(&mut file, &record2, b"", &keyring)?;

    let _ = handler.replay(&mut file, &keyring);

    let header = handler.read_subheader(&mut file, &keyring)?;

    println!("{:?}", header);

    Ok(())
}
