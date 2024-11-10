pub use self::op::*;
use crate::Kernel;

mod op;

/// Represents `vnode` structure.
pub trait Vnode<K: Kernel>: Sized {
    /// Returns `v_op`.
    fn ops(&self) -> &'static K::VopVector;
}

/// Represents `vop_vector` structure.
pub trait VopVector: Sized {}
