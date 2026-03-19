use zeroize::Zeroize;

use crate::errors::{Error, Result};
use crate::internal::memory::{lock_memory, unlock_memory};

#[derive(Zeroize, Default)]
pub struct SecretVec {
    inner: Vec<u8>,
}

impl SecretVec {
    pub fn new(mut data: Vec<u8>) -> Result<Self> {
        let ptr = data.as_mut_ptr();
        let len = data.capacity();

        if !lock_memory(ptr, len) {
            data.zeroize();
            return Err(Error::MemoryLockFailed);
        }

        Ok(Self { inner: data })
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl Drop for SecretVec {
    fn drop(&mut self) {
        self.inner.zeroize();

        let ptr = self.inner.as_mut_ptr();
        let cap = self.inner.capacity();

        unlock_memory(ptr, cap);
    }
}
