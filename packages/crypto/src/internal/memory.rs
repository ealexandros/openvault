#[cfg(unix)]
pub fn lock_memory(ptr: *mut u8, len: usize) -> bool {
    unsafe { libc::mlock(ptr.cast(), len) == 0 }
}

#[cfg(unix)]
pub fn unlock_memory(ptr: *mut u8, len: usize) {
    unsafe {
        libc::munlock(ptr.cast(), len);
    }
}

#[cfg(windows)]
pub fn lock_memory(ptr: *mut u8, len: usize) -> bool {
    use crate::internal::winapi::VirtualLock;
    unsafe { VirtualLock(ptr.cast(), len) != 0 }
}

#[cfg(windows)]
pub fn unlock_memory(ptr: *mut u8, len: usize) {
    use crate::internal::winapi::VirtualUnlock;
    unsafe {
        VirtualUnlock(ptr.cast(), len);
    }
}
