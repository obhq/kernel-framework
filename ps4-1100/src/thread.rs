use crate::ucred::Ucred;
use crate::Kernel;

/// Implementation of [`okf::thread::Thread`] for 11.00.
#[repr(C)]
pub struct Thread {
    pad1: [u8; 0x130],
    cred: *mut Ucred,
    pad2: [u8; 0x260],
    ret: [usize; 2], // td_retval
}

impl okf::thread::Thread<Kernel> for Thread {
    fn cred(&self) -> *mut Ucred {
        self.cred
    }

    fn ret(&self, i: usize) -> usize {
        self.ret[i]
    }
}
