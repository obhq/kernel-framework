use crate::queue::TailQueueEntry;

/// Represents `mount` structure.
pub trait Mount: Sized {
    /// Returns `mnt_list`.
    fn entry(&self) -> &TailQueueEntry<Self>;

    /// Returns mutable `mnt_list`.
    fn entry_mut(&mut self) -> &mut TailQueueEntry<Self>;

    /// Returns the value of `mnt_flag`.
    fn flags(&self) -> u64;
}
