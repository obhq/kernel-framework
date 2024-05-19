#![no_std]

use self::file::{File, OwnedFile};
use self::malloc::{Malloc, MallocFlags};
use self::socket::{SockAddr, Socket};
use self::thread::Thread;
use self::ucred::Ucred;
use self::uio::{Uio, UioSeg};
use core::ffi::{c_char, c_int};
use core::marker::PhantomData;
use core::num::NonZeroI32;
use core::ops::Deref;
use core::ptr::null_mut;
pub use okf_macros::*;

pub mod file;
pub mod malloc;
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
    const M_TEMP: StaticMut<Self::Malloc>;

    type File: File;
    type Malloc: Malloc;
    type Socket: Socket;
    type Thread: Thread;
    type Ucred: Ucred;
    type Uio: Uio<Self>;

    fn var<O: StaticOff>(self, off: O) -> O::Ops {
        let value = unsafe { self.mapped().as_ptr().add(off.value()) };

        <O::Ops as StaticOps>::new(value)
    }

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
    /// `ty` cannot be null.
    ///
    /// # Panics
    /// If `addr` is not valid.
    unsafe fn free(self, addr: *mut u8, ty: *mut Self::Malloc);

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
    /// `ty` cannot be null.
    unsafe fn malloc(self, size: usize, ty: *mut Self::Malloc, flags: MallocFlags) -> *mut u8;

    /// # Safety
    /// - `so` cannot be null.
    /// - `nam` cannot be null.
    /// - `td` cannot be null.
    unsafe fn sobind(
        self,
        so: *mut Self::Socket,
        nam: *mut SockAddr,
        td: *mut Self::Thread,
    ) -> c_int;

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

    /// # Safety
    /// - `so` cannot be null.
    /// - `td` cannot be null.
    unsafe fn solisten(self, so: *mut Self::Socket, backlog: c_int, td: *mut Self::Thread)
        -> c_int;
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

/// Offset of a static value in the kernel.
pub trait StaticOff: Copy {
    type Ops: StaticOps;

    fn value(self) -> usize;
}

/// Operations on a static value.
pub trait StaticOps: Copy {
    fn new(value: *const u8) -> Self;
}

/// Offset of an immutable static value in the kernel.
///
/// This immutable is only applied to the value itself at the offset. If the value is a pointer to
/// mutable data the pointer itself is immutable but the data it point to is mutable.
pub struct Static<T> {
    off: usize,
    phantom: PhantomData<T>,
}

impl<T> Static<T> {
    /// # Safety
    /// Behavior is undefined if `off` is not valid.
    pub const unsafe fn new(off: usize) -> Self {
        Self {
            off,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for Static<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Static<T> {}

impl<T> StaticOff for Static<T> {
    type Ops = ImmutableOps<T>;

    fn value(self) -> usize {
        self.off
    }
}

/// Implementation of [`StaticOps`] for [`Static`].
pub struct ImmutableOps<T>(*const T);

impl<T> StaticOps for ImmutableOps<T> {
    fn new(value: *const u8) -> Self {
        Self(value.cast())
    }
}

impl<T> Clone for ImmutableOps<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ImmutableOps<T> {}

impl<T> Deref for ImmutableOps<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

/// Offset of a mutable static value in the kernel.
///
/// This mutable is only applied to the value itself at the offset. If the value is a pointer to
/// immutable data the pointer itself is mutable but the data it point to is immutable.
pub struct StaticMut<T> {
    off: usize,
    phantom: PhantomData<T>,
}

impl<T> StaticMut<T> {
    /// # Safety
    /// Behavior is undefined if `off` is not valid.
    pub const unsafe fn new(off: usize) -> Self {
        Self {
            off,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for StaticMut<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for StaticMut<T> {}

impl<T> StaticOff for StaticMut<T> {
    type Ops = MutableOps<T>;

    fn value(self) -> usize {
        self.off
    }
}

/// Implementation of [`StaticOps`] for [`StaticMut`].
pub struct MutableOps<T>(*mut T);

impl<T> MutableOps<T> {
    pub fn ptr(self) -> *mut T {
        self.0
    }

    /// # Safety
    /// Behavior is undefined if write precondition is not upholds (e.g. the value required a lock
    /// before writing and the lock is not held by the calling thread).
    pub unsafe fn write(self, value: T) {
        self.0.write(value);
    }
}

impl<T: Copy> MutableOps<T> {
    /// # Safety
    /// Behavior is undefined if read precondition is not upholds (e.g. the value required a lock
    /// before reading and the lock is not held by the calling thread).
    pub unsafe fn read(self) -> T {
        self.0.read()
    }
}

impl<T> Clone for MutableOps<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for MutableOps<T> {}

impl<T> StaticOps for MutableOps<T> {
    fn new(value: *const u8) -> Self {
        Self(value as _)
    }
}
