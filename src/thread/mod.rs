use crate::Kernel;
use core::arch::asm;

/// Represents `thread` structure.
pub trait Thread<K: Kernel>: Sized {
    fn current() -> *mut Self {
        let mut p;

        unsafe {
            asm!("mov {}, gs:[0]", out(reg) p, options(readonly, pure, preserves_flags, nostack))
        };

        p
    }

    /// Returns value of `td_ucred`.
    fn cred(&self) -> *mut K::Ucred;

    /// Returns value of `td_retval[i]`.
    ///
    /// # Panics
    /// If `i` is not `0` or `1`.
    fn ret(&self, i: usize) -> usize;
}
