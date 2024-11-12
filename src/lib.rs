#![no_std]

use self::fd::OpenFlags;
use self::file::File;
use self::lock::{LockObject, Mtx};
use self::malloc::{Malloc, MallocFlags};
use self::mount::{Filesystem, FsOps, FsStats, Mount};
use self::pcpu::Pcpu;
use self::queue::TailQueue;
use self::socket::{SockAddr, Socket};
use self::thread::Thread;
use self::ucred::Ucred;
use self::uio::{Uio, UioSeg};
use self::vnode::{Vnode, VnodeOp, VopRead, VopUnlock, VopVector};
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::{c_char, c_int};
use core::marker::PhantomData;
use core::num::NonZero;
use core::ops::Deref;
use core::ptr::{null_mut, read_unaligned, write_unaligned};
pub use okf_macros::*;

pub mod fd;
pub mod file;
pub mod lock;
pub mod malloc;
pub mod mount;
pub mod pcpu;
pub mod queue;
pub mod socket;
pub mod thread;
pub mod ucred;
pub mod uio;
pub mod vnode;

#[cfg(fw = "1100")]
#[macro_export]
macro_rules! kernel {
    () => {
        okf_1100::Kernel
    };
}

/// Provides methods to access the PS4 kernel for a specific version.
///
/// All methods here are a direct call to the kernel so most of them are unsafe and hard to use.
/// Some modules may provide high-level wrappers that are easy to use.
pub trait Kernel: MappedKernel {
    const ACCEPT_MTX: StaticMut<Self::Mtx>;
    const EINTR: NonZero<c_int>;
    const EIO: NonZero<c_int>;
    const LK_EXCLUSIVE: c_int;
    const LK_SHARED: c_int;
    const M_TEMP: StaticMut<Self::Malloc>;
    const MBF_MNTLSTLOCK: c_int;
    const MBF_NOWAIT: c_int;
    const MNT_RDONLY: u64;
    const MOUNTLIST: StaticMut<TailQueue<Self::Mount>>;
    const MOUNTLIST_MTX: StaticMut<Self::Mtx>;
    const NOCPU: u32;
    const VOP_READ: StaticMut<Self::VnodeOp>;
    const VOP_UNLOCK: StaticMut<Self::VnodeOp>;

    type File: File;
    type Filesystem: Filesystem;
    type FsOps: FsOps<Self>;
    type FsStats: FsStats;
    type LockObject: LockObject;
    type Malloc: Malloc;
    type Mount: Mount<Self>;
    type Mtx: Mtx<Self>;
    type Pcpu: Pcpu<Self>;
    type Socket: Socket;
    type Thread: Thread<Self>;
    type Ucred: Ucred;
    type Uio: Uio<Self>;
    type Vnode: Vnode<Self>;
    type VnodeOp: VnodeOp;
    type VopRead: VopRead<Self>;
    type VopUnlock: VopUnlock;
    type VopVector: VopVector;

    fn var<O: StaticOff>(self, off: O) -> O::Ops {
        let value = unsafe { self.addr().add(off.value()) };

        <O::Ops as StaticOps>::new(value)
    }

    /// # Safety
    /// `fp` cannot be null.
    unsafe fn fget(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        fp: *mut *mut Self::File,
        mode: c_int,
        maxprotp: *mut u8,
    ) -> c_int;

    /// # Safety
    /// `fp` cannot be null.
    unsafe fn fget_write(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        unused: c_int,
        fp: *mut *mut Self::File,
    ) -> c_int;

    /// # Panics
    /// If [`File::refcnt()`] of `fp` is not zero.
    ///
    /// # Safety
    /// - `fp` cannot be null.
    unsafe fn fdrop(self, fp: *mut Self::File, td: *mut Self::Thread) -> c_int;

    /// # Safety
    /// `ty` cannot be null.
    ///
    /// # Panics
    /// If `addr` is not valid.
    unsafe fn free(self, addr: *mut u8, ty: *mut Self::Malloc);

    /// Note that this method return an errno, not a FD! You can grab the FD from `td_retval[0]` if
    /// this method return zero.
    ///
    /// # Safety
    /// - `td` cannot be null.
    /// - `path` cannot be null and must point to a null-terminated string if `seg` is [`UioSeg::Kernel`].
    unsafe fn kern_openat(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        path: *const c_char,
        seg: UioSeg,
        flags: OpenFlags,
        mode: c_int,
    ) -> c_int;

    /// # Safety
    /// `td` cannot be null.
    unsafe fn kern_close(self, td: *mut Self::Thread, fd: c_int) -> c_int;

    /// # Safety
    /// `td` cannot be null.
    unsafe fn kern_fsync(self, td: *mut Self::Thread, fd: c_int, fullsync: c_int) -> c_int;

    /// # Safety
    /// - `td` cannot be null.
    /// - `auio` cannot be null.
    unsafe fn kern_writev(self, td: *mut Self::Thread, fd: c_int, auio: *mut Self::Uio) -> c_int;

    /// The returned memory guarantee to be 8 byte aligment.
    ///
    /// # Safety
    /// `ty` cannot be null.
    unsafe fn malloc(self, size: usize, ty: *mut Self::Malloc, flags: MallocFlags) -> *mut u8;

    /// # Safety
    /// - `m` cannot be null.
    /// - `file` cannot be null and must point to a null-terminated string.
    unsafe fn mtx_lock_flags(
        self,
        m: *mut Self::Mtx,
        opts: c_int,
        file: *const c_char,
        line: c_int,
    );

    /// # Safety
    /// - `m` cannot be null.
    /// - `file` cannot be null and must point to a null-terminated string.
    unsafe fn mtx_unlock_flags(
        self,
        m: *mut Self::Mtx,
        opts: c_int,
        file: *const c_char,
        line: c_int,
    );

    /// # Safety
    /// - `ident` cannot be null.
    /// - `wmesg` cannot be null and must point to a null-terminated string.
    unsafe fn sleep(
        self,
        ident: *mut (),
        lock: *mut Self::LockObject,
        priority: c_int,
        wmesg: *const c_char,
        timo: c_int,
    ) -> c_int;

    /// # Safety
    /// - `so` cannot be null.
    /// - `nam` cannot be null.
    unsafe fn soaccept(self, so: *mut Self::Socket, nam: *mut *mut SockAddr) -> c_int;

    /// # Safety
    /// - `so` cannot be null.
    /// - `nam` cannot be null.
    /// - `td` cannot be null.
    unsafe fn sobind(
        self,
        so: *mut Self::Socket,
        nam: *mut SockAddr,
        td: *mut Self::Thread,
    ) -> c_int;

    /// # Safety
    /// `so` cannot be null.
    unsafe fn soclose(self, so: *mut Self::Socket) -> c_int;

    /// # Safety
    /// - `aso` cannot be null.
    /// - `cred` cannot be null.
    /// - `td` cannot be null.
    unsafe fn socreate(
        self,
        dom: c_int,
        aso: *mut *mut Self::Socket,
        ty: c_int,
        proto: c_int,
        cred: *mut Self::Ucred,
        td: *mut Self::Thread,
    ) -> c_int;

    /// # Safety
    /// - `so` cannot be null.
    /// - `td` cannot be null.
    unsafe fn solisten(self, so: *mut Self::Socket, backlog: c_int, td: *mut Self::Thread)
        -> c_int;

    /// # Safety
    /// `mp` cannot be null.
    unsafe fn vfs_busy(self, mp: *mut Self::Mount, flags: c_int) -> c_int;

    /// # Safety
    /// `mp` cannot be null.
    unsafe fn vfs_unbusy(self, mp: *mut Self::Mount);

    /// # Safety
    /// - `vec` cannot be null.
    /// - `args` cannot be null.
    unsafe fn vop_read(self, vec: *mut Self::VopVector, args: *mut Self::VopRead) -> c_int;

    /// # Safety
    /// - `vec` cannot be null.
    /// - `args` cannot be null.
    unsafe fn vop_unlock(self, vec: *mut Self::VopVector, args: *mut Self::VopUnlock) -> c_int;

    /// # Safety
    /// `vp` cannot be null and must be locked.
    unsafe fn vput(self, vp: *mut Self::Vnode);
}

/// Mapped PS4 kernel in the memory.
pub trait MappedKernel: Default + Sized + Copy + Send + Sync + 'static {
    /// Returns mapped address of the kernel.
    fn addr(self) -> *const u8;
}

/// Offset of a static value in the kernel.
pub trait StaticOff: Copy {
    type Ops: StaticOps;

    fn value(self) -> usize;
}

/// Operations on a static value.
pub trait StaticOps: Copy {
    fn new(value: *const u8) -> Self;
}

/// Offset of an immutable static value in the kernel.
///
/// This immutable is only applied to the value itself at the offset. If the value is a pointer to
/// mutable data the pointer itself is immutable but the data it point to is mutable.
pub struct Static<T> {
    off: usize,
    phantom: PhantomData<T>,
}

impl<T> Static<T> {
    /// # Safety
    /// Behavior is undefined if `off` is not valid.
    pub const unsafe fn new(off: usize) -> Self {
        Self {
            off,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for Static<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Static<T> {}

impl<T> StaticOff for Static<T> {
    type Ops = ImmutableOps<T>;

    fn value(self) -> usize {
        self.off
    }
}

/// Implementation of [`StaticOps`] for [`Static`].
pub struct ImmutableOps<T>(*const T);

impl<T> StaticOps for ImmutableOps<T> {
    fn new(value: *const u8) -> Self {
        Self(value.cast())
    }
}

impl<T> Clone for ImmutableOps<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ImmutableOps<T> {}

impl<T> Deref for ImmutableOps<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

/// Offset of a mutable static value in the kernel.
///
/// This mutable is only applied to the value itself at the offset. If the value is a pointer to
/// immutable data the pointer itself is mutable but the data it point to is immutable.
pub struct StaticMut<T> {
    off: usize,
    phantom: PhantomData<T>,
}

impl<T> StaticMut<T> {
    /// # Safety
    /// Behavior is undefined if `off` is not valid.
    pub const unsafe fn new(off: usize) -> Self {
        Self {
            off,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for StaticMut<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for StaticMut<T> {}

impl<T> StaticOff for StaticMut<T> {
    type Ops = MutableOps<T>;

    fn value(self) -> usize {
        self.off
    }
}

/// Implementation of [`StaticOps`] for [`StaticMut`].
pub struct MutableOps<T>(*mut T);

impl<T> MutableOps<T> {
    pub fn ptr(self) -> *mut T {
        self.0
    }

    /// # Safety
    /// Behavior is undefined if write precondition is not upholds (e.g. the value required a lock
    /// before writing and the lock is not held by the calling thread).
    pub unsafe fn write(self, value: T) {
        self.0.write(value);
    }
}

impl<T: Copy> MutableOps<T> {
    /// # Safety
    /// Behavior is undefined if read precondition is not upholds (e.g. the value required a lock
    /// before reading and the lock is not held by the calling thread).
    pub unsafe fn read(self) -> T {
        self.0.read()
    }
}

impl<T> Clone for MutableOps<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for MutableOps<T> {}

impl<T> StaticOps for MutableOps<T> {
    fn new(value: *const u8) -> Self {
        Self(value as _)
    }
}

/// Implementation of [`GlobalAlloc`] using `malloc` and `free` on `M_TEMP`.
pub struct Allocator<K: Kernel>(PhantomData<K>);

impl<K: Kernel> Allocator<K> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    /// # Safety
    /// `layout` must be non-zero.
    #[inline(never)]
    unsafe fn alloc(layout: Layout, flags: MallocFlags) -> *mut u8 {
        // Calculate allocation size to include a spare room for align adjustment.
        let size = if layout.align() <= 8 {
            layout.size()
        } else {
            match layout.size().checked_add(layout.align() - 8) {
                Some(v) => v,
                None => return null_mut(),
            }
        };

        // We will store how many bytes that we have shifted at the end.
        let size = match size.checked_add(size_of::<usize>()) {
            Some(v) => v,
            None => return null_mut(),
        };

        // Allocate.
        let k = K::default();
        let t = k.var(K::M_TEMP);
        let mem = k.malloc(size, t.ptr(), flags);

        if mem.is_null() {
            return null_mut();
        }

        // Get number of bytes to shift so the alignment is correct.
        let misaligned = (mem as usize) % layout.align();
        let adjust = if misaligned == 0 {
            0
        } else {
            layout.align() - misaligned
        };

        // Store how many bytes have been shifted.
        let mem = mem.add(adjust);

        write_unaligned(mem.add(layout.size()).cast(), adjust);

        mem
    }
}

impl<K: Kernel> Default for Allocator<K> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<K: Kernel> GlobalAlloc for Allocator<K> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Self::alloc(layout, MallocFlags::WAITOK)
    }

    #[inline(never)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Get original address before alignment.
        let adjusted: usize = read_unaligned(ptr.add(layout.size()).cast());
        let ptr = ptr.sub(adjusted);

        // Free the memory.
        let k = K::default();
        let t = k.var(K::M_TEMP);

        k.free(ptr, t.ptr());
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        Self::alloc(layout, MallocFlags::WAITOK | MallocFlags::ZERO)
    }
}
