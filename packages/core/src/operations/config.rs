use openvault_crypto::compression::CompressionAlgorithm;
use openvault_crypto::encryption::EncryptionAlgorithm;

use crate::vault::versions::factory::LATEST_FORMAT_VERSION;

#[derive(Debug)]
pub struct CreateConfig {
    pub version: u16,
    pub compression: CompressionAlgorithm,
    pub cipher: EncryptionAlgorithm,
    pub filename: String,
    pub overwrite: bool,
}

impl Default for CreateConfig {
    fn default() -> Self {
        Self {
            version: LATEST_FORMAT_VERSION,
            compression: CompressionAlgorithm::Zstd,
            cipher: EncryptionAlgorithm::XChaCha20Poly1305,
            filename: String::new(),
            overwrite: true,
        }
    }
}
