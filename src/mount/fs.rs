use core::ffi::c_char;

/// Represents `vfsconf` structure.
pub trait Filesystem: Sized {
    /// Returns `vfc_name`.
    fn name(&self) -> *const c_char;
}
