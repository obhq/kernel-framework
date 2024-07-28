use core::ffi::c_int;
use core::num::NonZero;

/// Encapsulates an errno value.
#[derive(Debug, Clone, Copy)]
pub struct Errno(NonZero<c_int>);

impl Errno {
    /// Returns [`None`] if `v` is zero.
    pub fn new(v: c_int) -> Option<Self> {
        NonZero::new(v).map(|v| Self(v))
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
