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

/// Represents `vop_readdir_args` structure.
pub trait VopReadDir<K: Kernel>: Sized {
    /// # Safety
    /// - `vp` cannot be null and must be locked.
    /// - `uio` cannot be null.
    /// - `cred` cannot be null.
    unsafe fn new(
        k: K,
        vp: *mut K::Vnode,
        uio: *mut K::Uio,
        cred: *mut K::Ucred,
        eof: *mut c_int,
        ncookies: *mut c_int,
        cookies: *mut *mut u64,
    ) -> Self;
}

/// Represents `vop_lookup_args` structure.
pub trait VopLookup<K: Kernel>: Sized {
    /// # Safety
    /// - `vp` cannot be null and must be locked.
    /// - `out` cannot be null.
    /// - `cn` cannot be null.
    unsafe fn new(
        k: K,
        vp: *mut K::Vnode,
        out: *mut *mut K::Vnode,
        cn: *mut K::ComponentName,
    ) -> Self;
}
