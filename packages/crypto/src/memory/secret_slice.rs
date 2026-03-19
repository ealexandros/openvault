use std::ops::{Deref, DerefMut};
use zeroize::Zeroize;

use crate::errors::{Error, Result};
use crate::internal::memory::{lock_memory, unlock_memory};

pub struct SecretSlice<const N: usize> {
    inner: Box<[u8; N]>,
}

impl<const N: usize> Zeroize for SecretSlice<N> {
    fn zeroize(&mut self) {
        self.inner.zeroize();
    }
}

impl<const N: usize> SecretSlice<N> {
    pub fn new(mut bytes: [u8; N]) -> Result<Self> {
        let mut boxed = Box::new(bytes);

        bytes.zeroize();

        if !lock_memory(boxed.as_mut_ptr(), boxed.len()) {
            boxed.zeroize();
            return Err(Error::MemoryLockFailed);
        }

        Ok(Self { inner: boxed })
    }
}

impl<const N: usize> AsRef<[u8; N]> for SecretSlice<N> {
    fn as_ref(&self) -> &[u8; N] {
        self.inner.as_ref()
    }
}

impl<const N: usize> Deref for SecretSlice<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const N: usize> DerefMut for SecretSlice<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<const N: usize> Drop for SecretSlice<N> {
    fn drop(&mut self) {
        self.inner.zeroize();
        unlock_memory(self.inner.as_mut_ptr(), self.inner.len());
    }
}
