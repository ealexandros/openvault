use std::fs::File;
use std::path::PathBuf;

use openvault_crypto::compression::CompressionAlgorithm;
use openvault_crypto::encryption::EncryptionAlgorithm;
use zeroize::Zeroize;

use crate::errors::Result;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::factory::FormatRef;
use crate::vault::versions::shared::format::FormatContext;

pub struct VaultSession {
    file: File,
    file_path: PathBuf,
    keyring: Keyring,
    compressor: CompressionAlgorithm,
    cipher: EncryptionAlgorithm,
    format: FormatRef,
}

impl VaultSession {
    pub fn new(
        file: File,
        file_path: PathBuf,
        keyring: Keyring,
        compressor: CompressionAlgorithm,
        cipher: EncryptionAlgorithm,
        format: FormatRef,
    ) -> Self {
        Self {
            file,
            file_path,
            keyring,
            compressor,
            cipher,
            format,
        }
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn name(&self) -> String {
        self.file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn file_mut(&mut self) -> &mut File {
        &mut self.file
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn size(&self) -> Result<u64> {
        Ok(self.file_path.metadata()?.len())
    }

    pub fn version(&self) -> u16 {
        self.format.version()
    }

    pub fn keyring(&self) -> &Keyring {
        &self.keyring
    }

    pub fn compressor(&self) -> CompressionAlgorithm {
        self.compressor
    }

    pub fn cipher(&self) -> EncryptionAlgorithm {
        self.cipher
    }

    pub fn format(&self) -> FormatRef {
        self.format
    }

    pub fn with_format_context<T>(
        &mut self,
        callback: impl FnOnce(&mut File, &FormatContext<'_>) -> Result<T>,
    ) -> Result<T> {
        let context = FormatContext::new(&self.keyring, self.compressor, self.cipher);
        callback(&mut self.file, &context)
    }
}

impl Zeroize for VaultSession {
    fn zeroize(&mut self) {
        self.keyring.zeroize();
    }
}
