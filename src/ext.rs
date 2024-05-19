use crate::file::OwnedFile;
use crate::Kernel;
use core::ffi::c_int;
use core::num::NonZeroI32;
use core::ptr::null_mut;

/// Provides wrapper methods for methods on [`Kernel`].
///
/// This trait is automatically implemented for any type that implement [`Kernel`].
pub trait KernelExt: Kernel {
    /// # Safety
    /// `td` should not be null although the PS4 does not use it currently.
    unsafe fn fget_write(
        self,
        td: *mut Self::Thread,
        fd: c_int,
    ) -> Result<OwnedFile<Self>, NonZeroI32>;
}

impl<T: Kernel> KernelExt for T {
    unsafe fn fget_write(
        self,
        td: *mut Self::Thread,
        fd: c_int,
    ) -> Result<OwnedFile<Self>, NonZeroI32> {
        let mut fp = null_mut();
        let errno = self.fget_write(td, fd, 0, &mut fp);

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(OwnedFile::new(self, fp)),
        }
    }
}
