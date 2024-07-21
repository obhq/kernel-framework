#![no_std]

use self::file::File;
use self::lock::{LockObject, Mtx};
use self::malloc::Malloc;
use self::mount::Mount;
use self::pcpu::Pcpu;
use self::socket::Socket;
use self::thread::Thread;
use self::ucred::Ucred;
use self::uio::Uio;
use core::ffi::{c_char, c_int};
use okf::fcntl::OpenFlags;
use okf::malloc::MallocFlags;
use okf::queue::TailQueue;
use okf::socket::SockAddr;
use okf::uio::UioSeg;
use okf::{offset, panic_handler, MappedKernel, StaticMut};

mod file;
mod lock;
mod malloc;
mod mount;
mod pcpu;
mod socket;
mod thread;
mod ucred;
mod uio;

/// Implementation of [`okf::Kernel`] for 11.00.
#[derive(Clone, Copy, MappedKernel)]
pub struct Kernel(*const u8);

impl okf::Kernel for Kernel {
    #[offset(0x221CCF8)]
    const ACCEPT_MTX: StaticMut<Self::Mtx>;
    #[offset(0x15415B0)]
    const M_TEMP: StaticMut<Self::Malloc>;
    #[offset(0x1A6AD60)]
    const MOUNTLIST: StaticMut<TailQueue<Self::Mount>>;
    #[offset(0x22D0F10)]
    const MOUNTLIST_MTX: StaticMut<Self::Mtx>;
    const NOCPU: u32 = 0xff;

    type File = File;
    type LockObject = LockObject;
    type Malloc = Malloc;
    type Mount = Mount;
    type Mtx = Mtx;
    type Pcpu = Pcpu;
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

    #[offset(0x1A43E0)]
    unsafe fn free(self, addr: *mut u8, ty: *mut Self::Malloc);

    #[offset(0xE63B0)]
    unsafe fn kern_openat(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        path: *const c_char,
        seg: UioSeg,
        flags: OpenFlags,
        mode: c_int,
    ) -> c_int;

    #[offset(0x416920)]
    unsafe fn kern_close(self, td: *mut Self::Thread, fd: c_int) -> c_int;

    #[offset(0xEAD50)]
    unsafe fn kern_fsync(self, td: *mut Self::Thread, fd: c_int, fullsync: c_int) -> c_int;

    #[offset(0xDD340)]
    unsafe fn kern_writev(self, td: *mut Self::Thread, fd: c_int, auio: *mut Self::Uio) -> c_int;

    #[offset(0x1A4220)]
    unsafe fn malloc(self, size: usize, ty: *mut Self::Malloc, flags: MallocFlags) -> *mut u8;

    #[offset(0x10E6A0)]
    unsafe fn mtx_lock_flags(
        self,
        m: *mut Self::Mtx,
        opts: c_int,
        file: *const c_char,
        line: c_int,
    );

    #[offset(0x10E950)]
    unsafe fn mtx_unlock_flags(
        self,
        m: *mut Self::Mtx,
        opts: c_int,
        file: *const c_char,
        line: c_int,
    );

    #[offset(0x365F50)]
    unsafe fn sleep(
        self,
        ident: *mut (),
        lock: *mut Self::LockObject,
        priority: c_int,
        wmesg: *const c_char,
        timo: c_int,
    ) -> c_int;

    #[offset(0x264AF0)]
    unsafe fn soaccept(self, so: *mut Self::Socket, nam: *mut *mut SockAddr) -> c_int;

    #[offset(0x264600)]
    unsafe fn sobind(
        self,
        so: *mut Self::Socket,
        nam: *mut SockAddr,
        td: *mut Self::Thread,
    ) -> c_int;

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

    #[offset(0x264620)]
    unsafe fn solisten(self, so: *mut Self::Socket, backlog: c_int, td: *mut Self::Thread)
        -> c_int;
}

unsafe impl Send for Kernel {}
unsafe impl Sync for Kernel {}

panic_handler!(0x1987C0);
