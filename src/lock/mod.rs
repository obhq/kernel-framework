use crate::Kernel;

/// Represents `lock_object` structure.
pub trait LockObject: Sized {}

/// Represents `mtx` structure.
pub trait Mtx<K: Kernel>: Sized {
    fn lock_mut(&mut self) -> &mut K::LockObject;
}
