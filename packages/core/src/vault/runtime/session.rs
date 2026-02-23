use std::fs::File;

use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::factory::FormatRef;

pub struct VaultSession {
    file: File,
    version: u16,
    keyring: Keyring,
    engine: FormatRef,
}

impl VaultSession {
    pub fn new(file: File, version: u16, keyring: Keyring, engine: FormatRef) -> Self {
        Self {
            file,
            version,
            keyring,
            engine,
        }
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn keyring(&self) -> &Keyring {
        &self.keyring
    }

    pub fn engine(&self) -> FormatRef {
        self.engine
    }
}
