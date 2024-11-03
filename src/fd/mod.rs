use crate::errno::Errno;
use crate::fcntl::OpenFlags;
use crate::pcpu::Pcpu;
use crate::thread::Thread;
use crate::uio::{IoVec, Uio, UioRw, UioSeg};
use crate::Kernel;
use core::ffi::{c_char, c_int};
use core::marker::PhantomData;

/// # Safety
/// `path` cannot be null and must point to a null-terminated string if `seg` is [`UioSeg::Kernel`].
pub unsafe fn openat<K: Kernel>(
    kern: K,
    fd: c_int,
    path: *const c_char,
    seg: UioSeg,
    flags: OpenFlags,
    mode: c_int,
) -> Result<OwnedFd<K>, Errno> {
    let td = K::Pcpu::curthread();
    let errno = kern.kern_openat(td, fd, path, seg, flags, mode);

    match Errno::new(errno) {
        Some(v) => Err(v),
        None => Ok(OwnedFd {
            kern,
            fd: (*td).ret(0).try_into().unwrap(),
            phantom: PhantomData,
        }),
    }
}

/// # Safety
/// - `buf` cannot be null and must be valid up to `len` if `seg` is [`UioSeg::Kernel`].
/// - `td` cannot be null.
pub unsafe fn write<K: Kernel>(
    kern: K,
    fd: c_int,
    buf: *const u8,
    seg: UioSeg,
    len: usize,
    td: *mut K::Thread,
) -> Result<usize, Errno> {
    // Setup iovec.
    let mut vec = IoVec {
        ptr: buf.cast_mut(),
        len,
    };

    // Write.
    let mut uio = K::Uio::new(td, UioRw::Write, seg, &mut vec, 1).unwrap();
    let errno = kern.kern_writev(td, fd, &mut uio);

    match Errno::new(errno) {
        Some(v) => Err(v),
        None => Ok((*td).ret(0)),
    }
}

/// RAII struct to call [`Kernel::kern_close()`] when dropped.
pub struct OwnedFd<K: Kernel> {
    kern: K,
    fd: c_int,
    phantom: PhantomData<*const ()>, // For !Send.
}

impl<K: Kernel> Drop for OwnedFd<K> {
    fn drop(&mut self) {
        // This drop must be called from the same process as the one that created the FD.
        assert_eq!(
            unsafe { self.kern.kern_close(K::Pcpu::curthread(), self.fd) },
            0
        );
    }
}
