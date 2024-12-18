use crate::Kernel;

/// Represents `uio` structure.
pub trait Uio<K: Kernel>: Sized {
    /// Returns [`None`] if [`IoVec::len`] of `iov` is greater than [`Uio::io_max()`].
    ///
    /// # Safety
    /// - `td` cannot be null.
    /// - `iov` cannot be null.
    unsafe fn write(iov: *mut IoVec, td: *mut K::Thread) -> Option<Self>;

    /// Returns [`None`] if [`IoVec::len`] of `iov` is greater than [`Uio::io_max()`].
    ///
    /// # Safety
    /// - `td` cannot be null.
    /// - `iov` cannot be null.
    ///
    /// # Panics
    /// If `off` larger than [`isize::MAX`].
    unsafe fn read(iov: *mut IoVec, off: usize, td: *mut K::Thread) -> Option<Self>;

    /// Returns value of `UIO_MAXIOV`.
    fn vec_max() -> usize {
        1024
    }

    /// Returns value of `IOSIZE_MAX`.
    fn io_max() -> usize {
        0x7fffffff
    }

    /// Returns `uio_offset`.
    fn offset(&self) -> isize;

    /// Returns `uio_resid`.
    fn remaining(&self) -> isize;
}

/// Represents `uio_seg` enum.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UioSeg {
    /// UIO_USERSPACE
    User,
    /// UIO_SYSSPACE
    Kernel,
}

/// Represents `uio_rw` enum.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UioRw {
    /// UIO_READ
    Read,
    /// UIO_WRITE
    Write,
}

/// Represents `iovec` structure.
#[repr(C)]
pub struct IoVec {
    pub ptr: *mut u8,
    pub len: usize,
}
