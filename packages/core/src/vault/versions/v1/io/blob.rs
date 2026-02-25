use std::io::Read;
use uuid::Uuid;

use crate::errors::{Error, Result};
use crate::features::shared::blob_ref::BlobRef;
use crate::internal::io_ext::{ReadWriter, Reader, SeekExt};
use crate::vault::versions::shared::traits::FormatContext;
use crate::vault::versions::v1::blob::{BlobChunkMeta, BlobManifest};
use crate::vault::versions::v1::io::aad::AadDomain;
use crate::vault::versions::v1::io::frame::{open_frame, seal_frame};
use crate::vault::versions::v1::mapper::{decode_manifest, encode_manifest};

const BLOB_CHUNK_SIZE: usize = 256 * 1024;
const BLOB_MANIFEST_VERSION: u16 = 1;

pub fn write_blob(
    rw: &mut ReadWriter,
    mut reader: impl Read,
    context: &FormatContext,
) -> Result<BlobRef> {
    rw.seek_to_end()?;

    let mut chunks = Vec::new();
    let mut total_size = 0u64;

    loop {
        let mut buffer = Vec::with_capacity(BLOB_CHUNK_SIZE);

        let n = reader
            .by_ref()
            .take(BLOB_CHUNK_SIZE as u64)
            .read_to_end(&mut buffer)? as u64;

        if n == 0 {
            break;
        }

        let offset = seal_frame(rw, AadDomain::BlobChunk, &buffer, context)?;
        chunks.push(BlobChunkMeta::new(offset, n as u32));
        total_size += n;
    }

    let id = Uuid::new_v4();
    let manifest = BlobManifest {
        version: BLOB_MANIFEST_VERSION,
        id,
        size_bytes: total_size,
        chunk_size: BLOB_CHUNK_SIZE as u32,
        chunks,
    };

    let manifest_bytes = encode_manifest(&manifest)?;
    let manifest_offset = seal_frame(rw, AadDomain::BlobManifest, &manifest_bytes, context)?;

    Ok(BlobRef::new(id, total_size, manifest_offset))
}

pub fn read_blob(
    reader: &mut Reader,
    blob_ref: &BlobRef,
    context: &FormatContext,
) -> Result<Vec<u8>> {
    reader.seek_from_start(blob_ref.manifest_offset)?;

    let manifest_bytes = open_frame(reader, AadDomain::BlobManifest, context)?;
    let manifest = decode_manifest(&manifest_bytes)?;

    if manifest.id != blob_ref.id || manifest.size_bytes != blob_ref.size_bytes {
        return Err(Error::InvalidVaultFormat);
    }

    let mut blob = Vec::with_capacity(manifest.size_bytes as usize);

    for chunk in &manifest.chunks {
        reader.seek_from_start(chunk.offset)?;

        let chunk_bytes = open_frame(reader, AadDomain::BlobChunk, context)?;

        if chunk_bytes.len() != chunk.size as usize {
            return Err(Error::InvalidVaultFormat);
        }

        blob.extend(chunk_bytes);
    }

    Ok(blob)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use openvault_crypto::compression::CompressionAlgorithm;
    use openvault_crypto::encryption::EncryptionAlgorithm;
    use openvault_crypto::keys::salt::Salt;

    use crate::vault::crypto::keyring::Keyring;
    use crate::vault::versions::shared::traits::FormatContext;
    use crate::vault::versions::v1::io::init_layout;

    use super::{read_blob, write_blob};

    fn test_keyring() -> Keyring {
        let salt = Salt::random();
        Keyring::derive(b"test-password", &salt).expect("failed to derive test keyring")
    }

    #[test]
    fn blob_roundtrip_chunked() {
        let keyring = test_keyring();
        let mut io = Cursor::new(Vec::new());
        let context = FormatContext::new(
            &keyring,
            CompressionAlgorithm::default(),
            EncryptionAlgorithm::default(),
        );
        init_layout(&mut io, &context).expect("init layout");

        let mut payload = Vec::new();
        for i in 0..(512 * 1024 + 157) {
            payload.push((i % 251) as u8);
        }

        let blob_ref = write_blob(&mut io, &mut payload.as_slice(), &context).expect("write blob");
        let restored = read_blob(&mut io, &blob_ref, &context).expect("read blob");

        assert_eq!(restored, payload);
    }

    #[test]
    fn blob_roundtrip_small() {
        let keyring = test_keyring();
        let mut io = Cursor::new(Vec::new());
        let context = FormatContext::new(
            &keyring,
            CompressionAlgorithm::default(),
            EncryptionAlgorithm::default(),
        );
        init_layout(&mut io, &context).expect("init layout");

        let payload = vec![1, 2, 3, 4, 5];

        let blob_ref = write_blob(&mut io, &mut payload.as_slice(), &context).expect("write blob");
        let restored = read_blob(&mut io, &blob_ref, &context).expect("read blob");

        assert_eq!(restored, payload);
    }
}
