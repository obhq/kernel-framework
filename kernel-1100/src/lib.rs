#![no_std]

use self::file::File;
use self::socket::Socket;
use self::thread::Thread;
use self::ucred::Ucred;
use self::uio::Uio;
use core::ffi::{c_char, c_int};
use okf::uio::UioSeg;
use okf::{offset, MappedKernel};

mod file;
mod socket;
mod thread;
mod ucred;
mod uio;

/// Implementation of [`okf::Kernel`] for 11.00.
#[derive(Clone, Copy, MappedKernel)]
pub struct Kernel(&'static [u8]);

impl okf::Kernel for Kernel {
    type File = File;
    type Socket = Socket;
    type Thread = Thread;
    type Ucred = Ucred;
    type Uio = Uio;

    #[offset(0x4191C0)]
    unsafe fn fget_write(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        unused: c_int,
        fp: *mut *mut Self::File,
    ) -> c_int;

    #[offset(0x4161B0)]
    unsafe fn fdrop(self, fp: *mut Self::File, td: *mut Self::Thread) -> c_int;

    #[offset(0xE63B0)]
    unsafe fn kern_openat(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        path: *const c_char,
        seg: UioSeg,
        flags: c_int,
        mode: c_int,
    ) -> c_int;

    #[offset(0x416920)]
    unsafe fn kern_close(self, td: *mut Self::Thread, fd: c_int) -> c_int;

    #[offset(0xEAD50)]
    unsafe fn kern_fsync(self, td: *mut Self::Thread, fd: c_int, fullsync: c_int) -> c_int;

    #[offset(0xDD340)]
    unsafe fn kern_writev(self, td: *mut Self::Thread, fd: c_int, auio: *mut Self::Uio) -> c_int;

    #[offset(0x264680)]
    unsafe fn soclose(self, so: *mut Self::Socket) -> c_int;

    #[offset(0x263890)]
    unsafe fn socreate(
        self,
        dom: c_int,
        aso: *mut *mut Self::Socket,
        ty: c_int,
        proto: c_int,
        cred: *mut Self::Ucred,
        td: *mut Self::Thread,
    ) -> c_int;
}
