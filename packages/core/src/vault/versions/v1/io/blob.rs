use crc32fast::Hasher;
use serde::{Deserialize, Serialize};

use crate::errors::{Error, Result};
use crate::features::blob_ref::BlobRef;
use crate::internal::io_ext::{Reader, Rw, SeekExt};
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};

const BLOB_CHUNK_SIZE: usize = 256 * 1024;
const BLOB_MANIFEST_VERSION: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlobChunkMeta {
    offset: u64,
    plain_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlobManifest {
    version: u16,
    id: [u8; 32],
    size_bytes: u64,
    chunk_size: u32,
    chunks: Vec<BlobChunkMeta>,
}

fn encode_manifest(manifest: &BlobManifest) -> Result<Vec<u8>> {
    postcard::to_allocvec(manifest).map_err(|_| Error::InvalidVaultFormat)
}

fn decode_manifest(bytes: &[u8]) -> Result<BlobManifest> {
    let manifest: BlobManifest =
        postcard::from_bytes(bytes).map_err(|_| Error::InvalidVaultFormat)?;
    if manifest.version != BLOB_MANIFEST_VERSION {
        return Err(Error::InvalidVaultFormat);
    }
    Ok(manifest)
}

fn compute_blob_id(blob: &[u8]) -> [u8; 32] {
    let mut id = [0u8; 32];
    for (i, word) in id.chunks_exact_mut(4).enumerate() {
        let mut hasher = Hasher::new();
        hasher.update(&(i as u32).to_le_bytes());
        hasher.update(blob);
        word.copy_from_slice(&hasher.finalize().to_le_bytes());
    }
    id
}

pub fn write_blob(rw: &mut Rw, blob: &[u8], keyring: &Keyring) -> Result<BlobRef> {
    rw.seek_end()?;

    let mut chunks = Vec::new();

    for chunk in blob.chunks(BLOB_CHUNK_SIZE) {
        let offset = seal_frame(rw, AadDomain::BlobChunk, chunk, keyring)?;
        chunks.push(BlobChunkMeta {
            offset,
            plain_size: chunk.len() as u32,
        });
    }

    let id = compute_blob_id(blob);
    let manifest = BlobManifest {
        version: BLOB_MANIFEST_VERSION,
        id,
        size_bytes: blob.len() as u64,
        chunk_size: BLOB_CHUNK_SIZE as u32,
        chunks,
    };

    let manifest_offset = seal_frame(
        rw,
        AadDomain::BlobManifest,
        &encode_manifest(&manifest)?,
        keyring,
    )?;

    Ok(BlobRef {
        id,
        size_bytes: blob.len() as u64,
        manifest_offset,
    })
}

pub fn read_blob(reader: &mut Reader, blob_ref: &BlobRef, keyring: &Keyring) -> Result<Vec<u8>> {
    reader.seek_from_start(blob_ref.manifest_offset)?;

    let manifest_bytes = open_frame(reader, AadDomain::BlobManifest, keyring)?;
    let manifest = decode_manifest(&manifest_bytes)?;

    if manifest.id != blob_ref.id || manifest.size_bytes != blob_ref.size_bytes {
        return Err(Error::InvalidVaultFormat);
    }

    let blob_capacity =
        usize::try_from(manifest.size_bytes).map_err(|_| Error::InvalidVaultFormat)?;
    let mut blob = Vec::with_capacity(blob_capacity);

    for chunk in &manifest.chunks {
        reader.seek_from_start(chunk.offset)?;
        let chunk_bytes = open_frame(reader, AadDomain::BlobChunk, keyring)?;

        if chunk_bytes.len() != chunk.plain_size as usize {
            return Err(Error::InvalidVaultFormat);
        }

        blob.extend_from_slice(&chunk_bytes);
    }

    if blob.len() as u64 != manifest.size_bytes {
        return Err(Error::InvalidVaultFormat);
    }

    Ok(blob)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::vault::crypto::keyring::Keyring;
    use crate::vault::versions::v1::io::init_layout;

    use super::{read_blob, write_blob};

    fn test_keyring() -> Keyring {
        let salt = openvault_crypto::keys::random_salt();
        Keyring::derive(b"test-password", &salt).expect("failed to derive test keyring")
    }

    #[test]
    fn blob_roundtrip_chunked() {
        let keyring = test_keyring();
        let mut io = Cursor::new(Vec::new());
        init_layout(&mut io, &keyring).expect("init layout");

        let mut payload = Vec::new();
        for i in 0..(512 * 1024 + 157) {
            payload.push((i % 251) as u8);
        }

        let blob_ref = write_blob(&mut io, &payload, &keyring).expect("write blob");
        let restored = read_blob(&mut io, &blob_ref, &keyring).expect("read blob");

        assert_eq!(restored, payload);
    }
}
