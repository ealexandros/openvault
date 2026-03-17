#[cfg(windows)]
extern "system" {
    fn VirtualLock(lpAddress: *mut u8, dwSize: usize) -> i32;
    fn VirtualUnlock(lpAddress: *mut u8, dwSize: usize) -> i32;
}
