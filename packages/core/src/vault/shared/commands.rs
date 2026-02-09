use std::path::Path;

use crate::crypto::compression::factory::CompressionAlgorithm;
use crate::crypto::encryption::factory::EncryptionAlgorithm;
use crate::errors::Result;
use crate::vault::v1::schema::vault::Vault;

pub trait VaultCommands {
    fn create(&self, source: &Path, password: &[u8], config: CreateConfig) -> Result;
    fn open(&self, path: &Path, password: &[u8], config: OpenConfig) -> Result<Vault>;
    fn update(&self, path: &Path, password: &[u8], config: UpdateConfig) -> Result;
    fn delete(&self, path: &Path, password: &[u8], config: DeleteConfig) -> Result;
    fn add(&self, path: &Path, password: &[u8], config: AddConfig) -> Result;
    fn compact(&self, path: &Path, password: &[u8], config: CompactConfig) -> Result;
}

#[derive(Debug)]
pub struct CreateConfig {
    pub compression: CompressionAlgorithm,
    pub cipher: EncryptionAlgorithm,
    pub output_path: String,
    pub filename: String,
    pub overwrite_existing: bool,
}

#[derive(Debug, Clone, Default)]
pub struct OpenConfig {}

#[derive(Debug, Clone, Default)]
pub struct UpdateConfig {}

#[derive(Debug, Clone, Default)]
pub struct AddConfig {}

#[derive(Debug, Clone, Default)]
pub struct DeleteConfig {
    pub target_id: u32,
}

#[derive(Debug, Clone, Default)]
pub struct CompactConfig {}

impl Default for CreateConfig {
    fn default() -> Self {
        Self {
            compression: CompressionAlgorithm::default(),
            cipher: EncryptionAlgorithm::default(),
            output_path: "./".to_string(),
            filename: "vault.ov".to_string(),
            overwrite_existing: false,
        }
    }
}
