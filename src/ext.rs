use crate::Kernel;
use core::ffi::c_int;
use core::num::NonZeroI32;

/// Provides wrapper methods for methods on [`Kernel`].
///
/// This trait is automatically implemented for any type that implement [`Kernel`].
pub trait KernelExt: Kernel {
    /// # Safety
    /// - `so` cannot be null.
    /// - `td` cannot be null.
    unsafe fn listen(
        self,
        so: *mut Self::Socket,
        backlog: c_int,
        td: *mut Self::Thread,
    ) -> Result<(), NonZeroI32>;
}

impl<T: Kernel> KernelExt for T {
    unsafe fn listen(
        self,
        so: *mut Self::Socket,
        backlog: c_int,
        td: *mut Self::Thread,
    ) -> Result<(), NonZeroI32> {
        let errno = self.solisten(so, backlog, td);

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(()),
        }
    }
}
