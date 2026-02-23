use std::fs::File;

use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::factory::FormatRef;

pub struct VaultSession {
    file: File,
    version: u16,
    keyring: Keyring,
    format: FormatRef,
}

impl VaultSession {
    pub fn new(file: File, version: u16, keyring: Keyring, format: FormatRef) -> Self {
        Self {
            file,
            version,
            keyring,
            format,
        }
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn file_mut(&mut self) -> &mut File {
        &mut self.file
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn keyring(&self) -> &Keyring {
        &self.keyring
    }

    pub fn format(&self) -> FormatRef {
        self.format
    }
}
