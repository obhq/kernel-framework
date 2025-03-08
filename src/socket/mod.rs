pub use self::inet::*;
use crate::Kernel;
use crate::thread::Thread;
use core::ffi::{c_int, c_short, c_ushort};
use core::num::NonZero;
use core::ptr::null_mut;

mod inet;

pub const AF_INET: c_int = 2;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;

/// # Safety
/// - `so` cannot be null.
/// - `td` cannot be null.
pub unsafe fn bind<K: Kernel>(
    kern: K,
    so: *mut K::Socket,
    nam: &mut SockAddr,
    td: *mut K::Thread,
) -> Result<(), NonZero<c_int>> {
    let errno = unsafe { kern.sobind(so, nam, td) };

    match NonZero::new(errno) {
        Some(v) => Err(v),
        None => Ok(()),
    }
}

/// # Safety
/// - `so` cannot be null.
/// - `td` cannot be null.
pub unsafe fn listen<K: Kernel>(
    kern: K,
    so: *mut K::Socket,
    backlog: c_int,
    td: *mut K::Thread,
) -> Result<(), NonZero<c_int>> {
    let errno = unsafe { kern.solisten(so, backlog, td) };

    match NonZero::new(errno) {
        Some(v) => Err(v),
        None => Ok(()),
    }
}

/// Represents `socket` structure.
pub trait Socket: Sized {
    /// Returns a value of `so_error`.
    fn error(&self) -> c_ushort;
    fn set_error(&self, v: c_ushort);

    /// Returns address of `so_timeo`.
    ///
    /// This address is used only as a sleep/wakeup address. Do not read or write a value to it.
    fn timeout(&self) -> *mut c_short;
}

/// RAII struct to call [`Kernel::soclose`] when dropped.
pub struct OwnedSocket<K: Kernel> {
    kern: K,
    sock: *mut K::Socket,
}

impl<K: Kernel> OwnedSocket<K> {
    /// # Safety
    /// `td` cannot be null.
    pub unsafe fn new(
        kern: K,
        dom: c_int,
        ty: c_int,
        proto: c_int,
        td: *mut K::Thread,
    ) -> Result<Self, NonZero<c_int>> {
        let mut sock = null_mut();
        let cred = unsafe { (*td).cred() };
        let errno = unsafe { kern.socreate(dom, &mut sock, ty, proto, cred, td) };

        match NonZero::new(errno) {
            Some(v) => Err(v),
            None => Ok(Self { kern, sock }),
        }
    }

    pub fn as_raw(&self) -> *mut K::Socket {
        self.sock
    }
}

impl<K: Kernel> Drop for OwnedSocket<K> {
    fn drop(&mut self) {
        // The kernel itself does not check if soclose is success so we don't need to.
        unsafe { self.kern.soclose(self.sock) };
    }
}

/// Represents `sockaddr` structure.
#[repr(C)]
pub struct SockAddr {
    pub sa_len: u8,
    pub sa_family: u8,
    pub sa_data: [u8; 14],
}
