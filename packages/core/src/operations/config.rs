use openvault_crypto::compression::CompressionAlgorithm;
use openvault_crypto::encryption::EncryptionAlgorithm;

use crate::vault::versions::factory::LATEST_FORMAT_VERSION;

#[derive(Debug, Clone)]
pub struct CreateConfig {
    pub version: u16,
    pub compression: CompressionAlgorithm,
    pub cipher: EncryptionAlgorithm,
    pub filename: String,
    pub overwrite: bool,
}

impl CreateConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_version(mut self, version: u16) -> Self {
        self.version = version;
        self
    }

    pub fn with_compression(mut self, compression: CompressionAlgorithm) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_encryption(mut self, cipher: EncryptionAlgorithm) -> Self {
        self.cipher = cipher;
        self
    }

    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = filename;
        self
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }
}

impl Default for CreateConfig {
    fn default() -> Self {
        Self {
            version: LATEST_FORMAT_VERSION,
            compression: CompressionAlgorithm::default(),
            cipher: EncryptionAlgorithm::default(),
            filename: String::new(),
            overwrite: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct OpenConfig {
    pub compression: CompressionAlgorithm,
    pub cipher: EncryptionAlgorithm,
}

impl OpenConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_compression(mut self, compression: CompressionAlgorithm) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_encryption(mut self, cipher: EncryptionAlgorithm) -> Self {
        self.cipher = cipher;
        self
    }

    pub fn from_create(config: &CreateConfig) -> Self {
        Self {
            compression: config.compression,
            cipher: config.cipher,
        }
    }
}
