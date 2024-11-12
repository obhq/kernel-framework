pub use self::dirent::*;
pub use self::op::*;
use crate::Kernel;
use core::ffi::c_int;

mod dirent;
mod op;

/// Represents `vnode` structure.
pub trait Vnode<K: Kernel>: Sized {
    /// Returns `v_type`.
    fn ty(&self) -> c_int;

    /// Returns `v_op`.
    fn ops(&self) -> *mut K::VopVector;
}

/// Represents `vop_vector` structure.
pub trait VopVector: Sized {}
