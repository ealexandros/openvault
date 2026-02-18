use openvault_crypto::keys::{DerivedKey, MKEY_SIZE, MasterKey, Salt};

use crate::errors::Result;

const CONTEXT_PREFIX: &str = "openvault";
const CONTEXT_ENVELOPE: &str = "openvault/core/envelope";

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

    pub fn envelope_key_bytes(&self) -> &[u8; MKEY_SIZE] {
        self.master.as_bytes()
    }

    pub fn derive_envelope_key<const N: usize>(&self) -> Result<DerivedKey<N>> {
        self.expand_context::<N>(CONTEXT_ENVELOPE)
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
