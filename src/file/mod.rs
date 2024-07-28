use crate::errno::Errno;
use crate::pcpu::Pcpu;
use crate::Kernel;
use core::ffi::c_int;
use core::ptr::null_mut;
use core::sync::atomic::{fence, AtomicU32, Ordering};

/// Represents `file` structure.
pub trait File: Sized {
    /// Returns `f_count` field.
    fn refcnt(&self) -> &AtomicU32;
}

/// RAII struct to decrease `file::f_count` when dropped.
pub struct OwnedFile<K: Kernel> {
    kern: K,
    file: *mut K::File,
}

impl<K: Kernel> OwnedFile<K> {
    /// # Safety
    /// `file` cannot be null and the caller must own a strong reference to it. This method do
    /// **not** increase the reference count of this file.
    pub unsafe fn new(kern: K, file: *mut K::File) -> Self {
        Self { kern, file }
    }

    pub fn from_fd(kern: K, fd: c_int) -> Result<Self, Errno> {
        let td = K::Pcpu::curthread();
        let mut fp = null_mut();
        let errno = unsafe { kern.fget(td, fd, &mut fp, 0, null_mut()) };

        match Errno::new(errno) {
            Some(v) => Err(v),
            None => Ok(Self { kern, file: fp }),
        }
    }
}

impl<K: Kernel> Drop for OwnedFile<K> {
    fn drop(&mut self) {
        // See Drop implementation on Arc how this thing work.
        if unsafe { (*self.file).refcnt().fetch_sub(1, Ordering::Release) } != 1 {
            return;
        }

        fence(Ordering::Acquire);

        // The kernel itself does not check if fdrop is success so we don't need to.
        unsafe { self.kern.fdrop(self.file, K::Pcpu::curthread()) };
    }
}
