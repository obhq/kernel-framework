use crate::file::OwnedFile;
use crate::socket::{OwnedSocket, SockAddr};
use crate::thread::Thread;
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

    /// # Safety
    /// - `td` cannot be null.
    unsafe fn socket(
        self,
        dom: c_int,
        ty: c_int,
        proto: c_int,
        td: *mut Self::Thread,
    ) -> Result<OwnedSocket<Self>, NonZeroI32>;

    /// # Safety
    /// - `so` cannot be null.
    /// - `td` cannot be null.
    unsafe fn bind(
        self,
        so: *mut Self::Socket,
        nam: &mut SockAddr,
        td: *mut Self::Thread,
    ) -> Result<(), NonZeroI32>;
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

    unsafe fn socket(
        self,
        dom: c_int,
        ty: c_int,
        proto: c_int,
        td: *mut Self::Thread,
    ) -> Result<OwnedSocket<Self>, NonZeroI32> {
        let mut so = null_mut();
        let cred = (*td).cred();
        let errno = self.socreate(dom, &mut so, ty, proto, cred, td);

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(OwnedSocket::new(self, so)),
        }
    }

    unsafe fn bind(
        self,
        so: *mut Self::Socket,
        nam: &mut SockAddr,
        td: *mut Self::Thread,
    ) -> Result<(), NonZeroI32> {
        let errno = self.sobind(so, nam, td);

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(()),
        }
    }
}
