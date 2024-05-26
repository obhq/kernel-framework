use crate::Kernel;

/// Represents `thread` structure.
pub trait Thread<K: Kernel>: Sized {
    /// Returns value of `td_ucred`.
    fn cred(&self) -> *mut K::Ucred;

    /// Returns value of `td_retval[i]`.
    ///
    /// # Panics
    /// If `i` is not `0` or `1`.
    fn ret(&self, i: usize) -> usize;
}
