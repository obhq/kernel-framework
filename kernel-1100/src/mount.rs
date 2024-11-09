use crate::lock::Mtx;
use crate::Kernel;
use okf::queue::TailQueueEntry;

/// Implementation of [`okf::mount::Mount`] for 11.00.
#[repr(C)]
pub struct Mount {
    mtx: Mtx,
    pad1: [u8; 0x8],
    entry: TailQueueEntry<Self>,
    pad2: [u8; 0x48],
    flags: u64,
}

impl okf::mount::Mount<Kernel> for Mount {
    fn mtx(&self) -> *mut Mtx {
        &self.mtx as *const Mtx as *mut Mtx
    }

    unsafe fn entry(&self) -> &TailQueueEntry<Self> {
        &self.entry
    }

    unsafe fn entry_mut(&mut self) -> &mut TailQueueEntry<Self> {
        &mut self.entry
    }

    unsafe fn flags(&self) -> u64 {
        self.flags
    }
}
