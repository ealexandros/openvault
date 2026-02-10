use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

use crate::errors::{Error, Result};
use crate::utils::io::ReadExt;
use crate::vault::v1::schema::header::{VAULT_HEADER_SIZE, VaultHeader};

const CRC_SIZE: usize = 4;

pub fn read_header_at_top(file: &mut File) -> Result<VaultHeader> {
    file.seek(SeekFrom::Start(0))?;

    let buf = file.read_exact_array::<{ VAULT_HEADER_SIZE }>()?;

    let (data, crc_bytes) = buf.split_at(buf.len() - CRC_SIZE);
    let stored_crc = u32::from_le_bytes(crc_bytes.try_into().unwrap());

    let computed_crc = crc32fast::hash(data);

    if stored_crc != computed_crc {
        return Err(Error::InvalidVaultChecksum);
    }

    postcard::from_bytes(data).map_err(|_| Error::InvalidVaultFormat)
}

pub fn write_header_at_top(header: &mut VaultHeader, file: &mut File) -> Result {
    file.seek(SeekFrom::Start(0))?;

    let mut data = postcard::to_stdvec(header).map_err(|_| Error::InvalidVaultFormat)?;

    const PAYLOAD_LEN: usize = VAULT_HEADER_SIZE - CRC_SIZE;

    if data.len() > PAYLOAD_LEN {
        return Err(Error::InvalidVaultFormat);
    }

    data.resize(PAYLOAD_LEN, 0);

    let crc = crc32fast::hash(&data);

    file.write_all(&data)?;
    file.write_all(&crc.to_le_bytes())?;

    header.crc = crc;
    Ok(())
}
