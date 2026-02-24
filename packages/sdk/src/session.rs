use std::io::{Cursor, Read};

use openvault_core::features::blob_ref::BlobRef;
use openvault_core::operations::blob::{get_blob, put_blob};
use openvault_core::operations::replay::replay_since_checkpoint;
use openvault_core::vault::runtime::VaultSession;
use openvault_core::vault::versions::shared::replay::ReplayState;

use crate::error::Result;

pub struct VaultHandle {
    inner: VaultSession,
}

impl VaultHandle {
    pub(crate) fn new(inner: VaultSession) -> Self {
        Self { inner }
    }

    pub fn version(&self) -> u16 {
        self.inner.version()
    }

    pub fn put_blob(&mut self, source: &mut dyn Read) -> Result<BlobRef> {
        put_blob(&mut self.inner, source).map_err(Into::into)
    }

    pub fn put_blob_bytes(&mut self, bytes: &[u8]) -> Result<BlobRef> {
        let mut cursor = Cursor::new(bytes);
        self.put_blob(&mut cursor)
    }

    pub fn get_blob(&mut self, blob_ref: &BlobRef) -> Result<Vec<u8>> {
        get_blob(&mut self.inner, blob_ref).map_err(Into::into)
    }

    pub fn replay_since_checkpoint(&mut self) -> Result<ReplayState> {
        replay_since_checkpoint(&mut self.inner).map_err(Into::into)
    }

    pub fn as_inner(&self) -> &VaultSession {
        &self.inner
    }

    pub fn as_inner_mut(&mut self) -> &mut VaultSession {
        &mut self.inner
    }

    pub fn into_inner(self) -> VaultSession {
        self.inner
    }
}
