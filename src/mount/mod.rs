use crate::queue::TailQueueEntry;
use crate::Kernel;

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

    /// Returns the value of `mnt_flag`.
    ///
    /// # Safety
    /// [`Mount::mtx()`] must be locked.
    unsafe fn flags(&self) -> u64;
}
