#![no_std]

use self::file::{File, OwnedFile};
use self::socket::Socket;
use self::thread::Thread;
use self::ucred::Ucred;
use self::uio::{Uio, UioSeg};
use core::ffi::{c_char, c_int};
use core::num::NonZeroI32;
use core::ptr::null_mut;
pub use okf_macros::*;

pub mod file;
pub mod socket;
pub mod thread;
pub mod ucred;
pub mod uio;

/// Provides methods to access the PS4 kernel for a specific version.
///
/// Most methods here are a direct call to the kernel so most of them are unsafe. A safe wrapper for
/// those methods are provides by [`KernelExt`], which is automatically implemented for any type
/// that implement [`Kernel`].
pub trait Kernel: MappedKernel {
    type File: File;
    type Socket: Socket;
    type Thread: Thread;
    type Ucred: Ucred;
    type Uio: Uio<Self>;

    /// # Safety
    /// `fp` cannot be null.
    unsafe fn fget_write(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        unused: c_int,
        fp: *mut *mut Self::File,
    ) -> c_int;

    /// # Panics
    /// If [`File::refcnt()`] of `fp` is not zero.
    ///
    /// # Safety
    /// - `fp` cannot be null.
    unsafe fn fdrop(self, fp: *mut Self::File, td: *mut Self::Thread) -> c_int;

    /// # Safety
    /// - `td` cannot be null.
    /// - `path` cannot be null and must point to a null-terminated string if `seg` is [`UioSeg::Kernel`].
    unsafe fn kern_openat(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        path: *const c_char,
        seg: UioSeg,
        flags: c_int,
        mode: c_int,
    ) -> c_int;

    /// # Safety
    /// `td` cannot be null.
    unsafe fn kern_close(self, td: *mut Self::Thread, fd: c_int) -> c_int;

    /// # Safety
    /// `td` cannot be null.
    unsafe fn kern_fsync(self, td: *mut Self::Thread, fd: c_int, fullsync: c_int) -> c_int;

    /// # Safety
    /// - `td` cannot be null.
    /// - `auio` cannot be null.
    unsafe fn kern_writev(self, td: *mut Self::Thread, fd: c_int, auio: *mut Self::Uio) -> c_int;

    /// # Safety
    /// `so` cannot be null.
    unsafe fn soclose(self, so: *mut Self::Socket) -> c_int;

    /// # Safety
    /// - `aso` cannot be null.
    /// - `cred` cannot be null.
    /// - `td` cannot be null.
    unsafe fn socreate(
        self,
        dom: c_int,
        aso: *mut *mut Self::Socket,
        ty: c_int,
        proto: c_int,
        cred: *mut Self::Ucred,
        td: *mut Self::Thread,
    ) -> c_int;
}

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

/// Mapped PS4 kernel in the memory.
pub trait MappedKernel: Sized + Copy + Send + Sync + 'static {
    /// # Safety
    /// `base` must point to a valid address of the kernel. Behavior is undefined if format of the
    /// kernel is unknown.
    ///
    /// # Panics
    /// This function may panic if format of the kernel is unknown.
    unsafe fn new(base: *const u8) -> Self;

    /// Returns mapped memory of the kernel.
    ///
    /// # Safety
    /// The returned slice can contains `PF_W` programs. That mean the memory covered by this slice
    /// can mutate at any time. The whole slice is guarantee to be readable.
    unsafe fn mapped(self) -> &'static [u8];
}
