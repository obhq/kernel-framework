use crate::Kernel;
use core::ffi::c_int;

/// Represents `vnodeop_desc` structure.
pub trait VnodeOp: Sized {}

/// Represents `vop_unlock_args` structure.
pub trait VopUnlock: Sized {}

/// Represents `vop_read_args` structure.
pub trait VopRead<K: Kernel>: Sized {
    /// # Safety
    /// - `vp` cannot be null and must be locked.
    /// - `uio` cannot be null.
    /// - `cred` cannot be null.
    unsafe fn new(
        k: K,
        vp: *mut K::Vnode,
        uio: *mut K::Uio,
        flags: c_int,
        cred: *mut K::Ucred,
    ) -> Self;
}
