use crate::thread::Thread;
use crate::Kernel;
use core::ffi::c_int;
use okf::uio::{IoVec, UioRw, UioSeg};

/// Implementation of [`okf::uio::Uio`] for 11.00.
#[repr(C)]
pub struct Uio {
    iov: *mut IoVec,
    len: c_int,
    off: isize,
    res: isize,
    seg: UioSeg,
    op: UioRw,
    td: *mut Thread,
}

impl okf::uio::Uio<Kernel> for Uio {
    unsafe fn write(td: *mut Thread, iov: *mut IoVec) -> Option<Self> {
        let res = (*iov).len;

        if res > Self::io_max() {
            return None;
        }

        Some(Self {
            iov,
            len: 1,
            off: -1,
            res: res.try_into().unwrap(),
            seg: UioSeg::Kernel,
            op: UioRw::Write,
            td,
        })
    }

    unsafe fn read(td: *mut Thread, iov: *mut IoVec) -> Option<Self> {
        let res = (*iov).len;

        if res > Self::io_max() {
            return None;
        }

        Some(Self {
            iov,
            len: 1,
            off: 0,
            res: res.try_into().unwrap(),
            seg: UioSeg::Kernel,
            op: UioRw::Read,
            td,
        })
    }

    fn offset(&self) -> isize {
        self.off
    }

    fn remaining(&self) -> isize {
        self.res
    }
}
