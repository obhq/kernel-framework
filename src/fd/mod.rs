use crate::pcpu::Pcpu;
use crate::thread::Thread;
use crate::uio::{IoVec, Uio, UioRw, UioSeg};
use crate::Kernel;
use bitflags::bitflags;
use core::ffi::{c_char, c_int};
use core::marker::PhantomData;
use core::num::NonZero;

pub const AT_FDCWD: c_int = -100;

/// # Safety
/// `path` cannot be null and must point to a null-terminated string if `seg` is [`UioSeg::Kernel`].
#[inline(never)]
pub unsafe fn openat<K: Kernel>(
    kern: K,
    fd: c_int,
    path: *const c_char,
    seg: UioSeg,
    flags: OpenFlags,
    mode: c_int,
) -> Result<OwnedFd<K>, NonZero<c_int>> {
    let td = K::Pcpu::curthread();
    let errno = kern.kern_openat(td, fd, path, seg, flags, mode);

    match NonZero::new(errno) {
        Some(v) => Err(v),
        None => Ok(OwnedFd {
            kern,
            fd: (*td).ret(0).try_into().unwrap(),
            phantom: PhantomData,
        }),
    }
}

/// # Safety
/// `td` cannot be null.
#[inline(never)]
pub unsafe fn write_all<K: Kernel>(
    kern: K,
    fd: c_int,
    mut data: &[u8],
    seg: UioSeg,
    td: *mut K::Thread,
) -> Result<(), NonZero<c_int>> {
    while !data.is_empty() {
        let written = match write(kern, fd, data, seg, td) {
            Ok(v) => v,
            Err(e) if e == K::EINTR => continue,
            Err(e) => return Err(e),
        };

        if written == 0 {
            return Err(K::EIO);
        }

        data = &data[written..];
    }

    Ok(())
}

/// # Safety
/// `td` cannot be null.
#[inline(never)]
pub unsafe fn write<K: Kernel>(
    kern: K,
    fd: c_int,
    data: &[u8],
    seg: UioSeg,
    td: *mut K::Thread,
) -> Result<usize, NonZero<c_int>> {
    // Setup iovec.
    let mut vec = IoVec {
        ptr: data.as_ptr().cast_mut(),
        len: data.len(),
    };

    // Write.
    let mut uio = K::Uio::new(td, UioRw::Write, seg, &mut vec, 1).unwrap();
    let errno = kern.kern_writev(td, fd, &mut uio);

    match NonZero::new(errno) {
        Some(v) => Err(v),
        None => Ok((*td).ret(0)),
    }
}

bitflags! {
    /// Flags for `open` and related functions.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OpenFlags: c_int {
        const O_RDONLY = 0x00000000;
        const O_WRONLY = 0x00000001;
        const O_RDWR = 0x00000002;
        const O_ACCMODE = Self::O_WRONLY.bits() | Self::O_RDWR.bits();
        const O_SHLOCK = 0x00000010;
        const O_EXLOCK = 0x00000020;
        const O_CREAT = 0x00000200;
        const O_TRUNC = 0x00000400;
        const O_EXCL = 0x00000800;
        const O_EXEC = 0x00040000;
        const O_CLOEXEC = 0x00100000;
    }
}

/// RAII struct to call [`Kernel::kern_close()`] when dropped.
pub struct OwnedFd<K: Kernel> {
    kern: K,
    fd: c_int,
    phantom: PhantomData<*const ()>, // For !Send.
}

impl<K: Kernel> OwnedFd<K> {
    pub fn as_raw_fd(&self) -> c_int {
        self.fd
    }
}

impl<K: Kernel> Drop for OwnedFd<K> {
    #[inline(never)]
    fn drop(&mut self) {
        // This drop must be called from the same process as the one that created the FD.
        assert_eq!(
            unsafe { self.kern.kern_close(K::Pcpu::curthread(), self.fd) },
            0
        );
    }
}
