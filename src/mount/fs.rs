use crate::Kernel;
use core::ffi::{c_char, c_int};
use core::num::NonZero;

/// Represents `vfsconf` structure.
pub trait Filesystem: Sized {
    /// Returns `vfc_name`.
    fn name(&self) -> *const c_char;
}

/// Represents `vfsops` structure.
pub trait FsOps<K: Kernel>: Sized {
    /// Invoke `vfs_root`.
    ///
    /// # Safety
    /// `mp` cannot be null.
    unsafe fn root(&self, mp: *mut K::Mount, flags: c_int)
    -> Result<*mut K::Vnode, NonZero<c_int>>;
}
