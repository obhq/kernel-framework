use crate::Kernel;

/// Represents `socket` structure.
pub trait Socket: Sized {}

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
}

impl<K: Kernel> Drop for OwnedSocket<K> {
    fn drop(&mut self) {
        // The kernel itself does not check if soclose is success so we don't need to.
        unsafe { self.kernel.soclose(self.sock) };
    }
}
