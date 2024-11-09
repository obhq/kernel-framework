use crate::lock::Mtx;
use crate::Kernel;
use core::ffi::c_char;
use okf::queue::TailQueueEntry;

/// Implementation of [`okf::mount::Mount`] for 11.00.
#[repr(C)]
pub struct Mount {
    mtx: Mtx,
    pad1: [u8; 0x8],
    entry: TailQueueEntry<Self>,
    pad2: [u8; 8],
    fs: *mut Filesystem,
    pad3: [u8; 0x38],
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

    fn fs(&self) -> *mut Filesystem {
        self.fs
    }

    unsafe fn flags(&self) -> u64 {
        self.flags
    }
}

/// Implementation of [`okf::mount::Filesystem`] for 11.00.
#[repr(C)]
pub struct Filesystem {
    pad1: [u8; 4],
    name: [c_char; 16],
}

impl okf::mount::Filesystem for Filesystem {
    fn name(&self) -> *const c_char {
        self.name.as_ptr()
    }
}
