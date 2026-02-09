use crate::constants::KEY_LEN;
use crate::crypto::kdf;
use crate::errors::Result;
use strum_macros::EnumIter;
use zeroize::Zeroizing;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum VaultKeys {
    Metadata,
    Files,
}

impl VaultKeys {
    fn info(self) -> &'static [u8] {
        match self {
            VaultKeys::Metadata => b"vault-metadata-v1",
            VaultKeys::Files => b"vault-files-v1",
        }
    }

    pub fn derive(self, master_key: &Zeroizing<[u8; KEY_LEN]>) -> Result<Zeroizing<[u8; KEY_LEN]>> {
        kdf::derive_subkey(master_key.as_slice(), self.info())
    }
}
