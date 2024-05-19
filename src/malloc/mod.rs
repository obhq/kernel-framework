use bitflags::bitflags;
use core::ffi::c_int;

/// Represents `malloc_type` structure.
pub trait Malloc: Sized {}

bitflags! {
    /// Flags to `malloc`.
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct MallocFlags: c_int {
        const WAITOK = 0x0002;
    }
}
