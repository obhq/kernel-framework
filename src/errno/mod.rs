use core::ffi::c_int;
use core::num::NonZero;

pub const EINTR: Errno = unsafe { Errno(NonZero::new_unchecked(4)) };
pub const EIO: Errno = unsafe { Errno(NonZero::new_unchecked(5)) };

/// Encapsulates an errno value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Errno(NonZero<c_int>);

impl Errno {
    /// Returns [`None`] if `v` is zero.
    pub fn new(v: c_int) -> Option<Self> {
        NonZero::new(v).map(Self)
    }
}

impl From<Errno> for c_int {
    fn from(value: Errno) -> Self {
        value.0.get()
    }
}

impl From<Errno> for NonZero<c_int> {
    fn from(value: Errno) -> Self {
        value.0
    }
}
