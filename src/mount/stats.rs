use core::ffi::c_char;

/// Represents `statfs` structure.
pub trait FsStats: Sized {
    /// Returns `f_mntfromname`.
    fn mounted_from(&self) -> *const c_char;
}
