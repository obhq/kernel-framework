use crate::Kernel;
use core::ffi::{c_char, c_int};

/// Represents `componentname` structure.
pub trait ComponentName<K: Kernel>: Sized {
    /// # Safety
    /// - `buf` cannot be null and must point to a null-terminated string.
    /// - `td` cannot be null.
    unsafe fn new(k: K, op: u64, lk: c_int, buf: *mut c_char, td: *mut K::Thread) -> Self;
}
