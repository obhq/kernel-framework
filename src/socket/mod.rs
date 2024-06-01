pub use self::inet::*;
use crate::Kernel;
use core::ffi::{c_int, c_short, c_ushort};

mod inet;

pub const AF_INET: c_int = 2;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;

/// Represents `socket` structure.
pub trait Socket: Sized {
    /// Returns a value of `so_error`.
    fn error(&self) -> c_ushort;

    /// Returns a mutable reference to `so_timeo`.
    fn timeout_mut(&mut self) -> &mut c_short;
}

/// RAII struct to call [`Kernel::soclose`] when dropped.
pub struct OwnedSocket<K: Kernel> {
    kernel: K,
    sock: *mut K::Socket,
}

impl<K: Kernel> OwnedSocket<K> {
    /// # Safety
    /// `sock` cannot be null and the caller must be an owner.
    pub unsafe fn new(kernel: K, sock: *mut K::Socket) -> Self {
        Self { kernel, sock }
    }

    pub fn as_raw(&self) -> *mut K::Socket {
        self.sock
    }
}

impl<K: Kernel> Drop for OwnedSocket<K> {
    fn drop(&mut self) {
        // The kernel itself does not check if soclose is success so we don't need to.
        unsafe { self.kernel.soclose(self.sock) };
    }
}

/// Represents `sockaddr` structure.
#[repr(C)]
pub struct SockAddr {
    pub sa_len: u8,
    pub sa_family: u8,
    pub sa_data: [u8; 14],
}
