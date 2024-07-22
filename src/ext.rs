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
