use crate::queue::TailQueueEntry;
use crate::Kernel;
use core::ffi::c_char;

/// Represents `mount` structure.
pub trait Mount<K: Kernel>: Sized {
    /// Returns `mnt_mtx`.
    fn mtx(&self) -> *mut K::Mtx;

    /// Returns `mnt_list`.
    ///
    /// # Safety
    /// [`Kernel::MOUNTLIST_MTX`] must be locked.
    unsafe fn entry(&self) -> &TailQueueEntry<Self>;

    /// Returns mutable `mnt_list`.
    ///
    /// # Safety
    /// [`Kernel::MOUNTLIST_MTX`] must be locked.
    unsafe fn entry_mut(&mut self) -> &mut TailQueueEntry<Self>;

    /// Returns `mnt_vfc`.
    fn fs(&self) -> *mut K::Filesystem;

    /// Returns the value of `mnt_flag`.
    ///
    /// # Safety
    /// [`Mount::mtx()`] must be locked.
    unsafe fn flags(&self) -> u64;
}

/// Represents `vfsconf` structure.
pub trait Filesystem: Sized {
    /// Returns `vfc_name`.
    fn name(&self) -> *const c_char;
}
