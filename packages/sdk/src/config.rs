use openvault_core::operations::config::CreateConfig;
use openvault_crypto::compression::CompressionAlgorithm;
use openvault_crypto::encryption::EncryptionAlgorithm;

#[derive(Debug, Clone, Default)]
pub struct CreateVaultOptions {
    inner: CreateConfig,
}

impl CreateVaultOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_version(mut self, version: u16) -> Self {
        self.inner.version = version;
        self
    }

    pub fn with_compression(mut self, compression: CompressionAlgorithm) -> Self {
        self.inner.compression = compression;
        self
    }

    pub fn with_encryption(mut self, cipher: EncryptionAlgorithm) -> Self {
        self.inner.cipher = cipher;
        self
    }

    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.inner.filename = filename.into();
        self
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.inner.overwrite = overwrite;
        self
    }

    pub(crate) fn into_core(self) -> CreateConfig {
        self.inner
    }
}
