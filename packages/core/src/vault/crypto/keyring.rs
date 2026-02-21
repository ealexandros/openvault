use openvault_crypto::keys::derived_key::DerivedKey;
use openvault_crypto::keys::master_key::MasterKey;
use openvault_crypto::keys::salt::Salt;

use crate::errors::Result;

const CONTEXT_PREFIX: &str = "openvault";
const CONTEXT_META: &str = "openvault/meta";
const CONTEXT_DEFAULT: &str = "openvault/default";
const CONTEXT_BLOB_MANIFEST: &str = "openvault/blob/manifest";
const CONTEXT_BLOB_CHUNK: &str = "openvault/blob/chunk";

#[derive(Debug, Clone)]
pub struct Keyring {
    master: MasterKey,
}

impl Keyring {
    pub fn new(master: MasterKey) -> Self {
        Self { master }
    }

    pub fn derive(password: &[u8], salt: &Salt) -> Result<Self> {
        let master = MasterKey::derive(password, salt)?;
        Ok(Self { master })
    }

    pub fn master_key(&self) -> &MasterKey {
        &self.master
    }

    pub fn derive_meta_key(&self) -> Result<DerivedKey<32>> {
        self.expand_context(CONTEXT_META)
    }

    pub fn derive_default_key(&self) -> Result<DerivedKey<32>> {
        self.expand_context(CONTEXT_DEFAULT)
    }

    pub fn derive_blob_manifest_key(&self) -> Result<DerivedKey<32>> {
        self.expand_context(CONTEXT_BLOB_MANIFEST)
    }

    pub fn derive_blob_chunk_key(&self) -> Result<DerivedKey<32>> {
        self.expand_context(CONTEXT_BLOB_CHUNK)
    }

    pub fn derive_feature_key<const N: usize>(&self, feature: &str) -> Result<DerivedKey<N>> {
        let context = format!("{}/feature/{}", CONTEXT_PREFIX, feature);
        self.expand_context::<N>(&context)
    }

    #[inline]
    fn expand_context<const N: usize>(&self, context: &str) -> Result<DerivedKey<N>> {
        self.master.expand(context.as_bytes()).map_err(Into::into)
    }
}
