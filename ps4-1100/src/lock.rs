use crate::Kernel;

/// Implementation of [`okf::lock::LockObject`] for 11.00.
#[repr(C)]
pub struct LockObject {
    pad1: [u8; 0x18],
}

impl okf::lock::LockObject for LockObject {}

/// Implementation of [`okf::lock::Mtx`] for 11.00.
#[repr(C)]
pub struct Mtx {
    lock: LockObject,
    state: usize,
}

impl okf::lock::Mtx<Kernel> for Mtx {
    fn lock_mut(&mut self) -> &mut LockObject {
        &mut self.lock
    }
}
