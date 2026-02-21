use openvault_crypto::keys::derived_key::DerivedKey;

use crate::errors::Result;
use crate::vault::crypto::keyring::Keyring;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum AadDomain {
    Subheader,
    Checkpoint,
    Record,
    BlobManifest,
    BlobChunk,
}

impl AadDomain {
    pub fn encode(&self, offset: u64) -> Vec<u8> {
        let mut aad = b"openvault/v1/".to_vec();

        aad.push(match self {
            AadDomain::Subheader => 1,
            AadDomain::Checkpoint => 2,
            AadDomain::Record => 3,
            AadDomain::BlobManifest => 4,
            AadDomain::BlobChunk => 5,
        });
        aad.extend_from_slice(&offset.to_le_bytes());

        aad
    }

    pub fn derive_key(&self, keyring: &Keyring) -> Result<DerivedKey> {
        match self {
            AadDomain::Subheader | AadDomain::Checkpoint | AadDomain::Record => {
                keyring.derive_meta_key()
            }

            AadDomain::BlobManifest => keyring.derive_blob_manifest_key(),
            AadDomain::BlobChunk => keyring.derive_blob_chunk_key(),
        }
    }
}
