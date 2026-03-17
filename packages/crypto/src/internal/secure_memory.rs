use std::ops::{Deref, DerefMut};
use zeroize::Zeroize;

use crate::errors::{Error, Result};

#[cfg(unix)]
fn lock_memory(ptr: *mut u8, len: usize) -> bool {
    unsafe { libc::mlock(ptr.cast(), len) == 0 }
}

#[cfg(unix)]
fn unlock_memory(ptr: *mut u8, len: usize) {
    unsafe {
        libc::munlock(ptr.cast(), len);
    }
}

#[cfg(windows)]
fn lock_memory(ptr: *mut u8, len: usize) -> bool {
    use crate::internal::winapi::VirtualLock;
    unsafe { VirtualLock(ptr.cast(), len) != 0 }
}

#[cfg(windows)]
fn unlock_memory(ptr: *mut u8, len: usize) {
    use crate::internal::winapi::VirtualUnlock;
    unsafe {
        VirtualUnlock(ptr.cast(), len);
    }
}

pub struct SecureMemory<const N: usize> {
    inner: Box<[u8; N]>,
}

impl<const N: usize> Zeroize for SecureMemory<N> {
    fn zeroize(&mut self) {
        self.inner.zeroize();
    }
}

impl<const N: usize> SecureMemory<N> {
    pub fn new(mut bytes: [u8; N]) -> Result<Self> {
        let mut boxed = Box::new(bytes);

        bytes.zeroize();

        if !lock_memory(boxed.as_mut_ptr(), boxed.len()) {
            boxed.zeroize();
            return Err(Error::MemoryLockFailed);
        }

        Ok(Self { inner: boxed })
    }

    pub fn as_ref(&self) -> &[u8; N] {
        &self.inner
    }
}

impl<const N: usize> Deref for SecureMemory<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const N: usize> DerefMut for SecureMemory<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<const N: usize> Drop for SecureMemory<N> {
    fn drop(&mut self) {
        self.inner.zeroize();
        unlock_memory(self.inner.as_mut_ptr(), self.inner.len());
    }
}
