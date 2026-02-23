use openvault_crypto::keys::derived_key::DerivedKey;
use openvault_crypto::keys::master_key::MasterKey;
use openvault_crypto::keys::salt::Salt;

use crate::errors::Result;

const CONTEXT_PREFIX: &str = "openvault";

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

    pub fn derive_meta_key(&self, version: u16) -> Result<DerivedKey> {
        let context = format!("{}/v{}/meta", CONTEXT_PREFIX, version);
        self.expand_context(&context)
    }

    pub fn derive_default_key(&self, version: u16) -> Result<DerivedKey> {
        let context = format!("{}/v{}/default", CONTEXT_PREFIX, version);
        self.expand_context(&context)
    }

    pub fn derive_blob_manifest_key(&self, version: u16) -> Result<DerivedKey> {
        let context = format!("{}/v{}/blob/manifest", CONTEXT_PREFIX, version);
        self.expand_context(&context)
    }

    pub fn derive_blob_chunk_key(&self, version: u16) -> Result<DerivedKey> {
        let context = format!("{}/v{}/blob/chunk", CONTEXT_PREFIX, version);
        self.expand_context(&context)
    }

    pub fn derive_feature_key(&self, version: u16, feature: &str) -> Result<DerivedKey> {
        let context = format!("{}/v{}/feature/{}", CONTEXT_PREFIX, version, feature);
        self.expand_context(&context)
    }

    #[inline]
    fn expand_context(&self, context: &str) -> Result<DerivedKey> {
        self.master.expand(context.as_bytes()).map_err(Into::into)
    }
}
