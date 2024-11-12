use crate::ucred::Ucred;
use crate::uio::Uio;
use core::ffi::c_int;
use okf::Kernel;

/// Implementation of [`okf::vnode::Vnode`] for 11.00.
#[repr(C)]
pub struct Vnode {
    pad1: [u8; 0x10],
    ops: *mut VopVector,
}

impl okf::vnode::Vnode<crate::Kernel> for Vnode {
    fn ops(&self) -> *mut VopVector {
        self.ops
    }
}

/// Implementation of [`okf::vnode::VopVector`] for 11.00.
#[repr(C)]
pub struct VopVector {}

impl okf::vnode::VopVector for VopVector {}

/// Implementation of [`okf::vnode::VnodeOp`] for 11.00.
#[repr(C)]
pub struct VnodeOp {}

impl okf::vnode::VnodeOp for VnodeOp {}

/// Implementation of [`okf::vnode::VopUnlock`] for 11.00.
#[repr(C)]
pub struct VopUnlock {
    desc: *mut VnodeOp,
    vp: *mut Vnode,
    flags: c_int,
}

impl okf::vnode::VopUnlock for VopUnlock {}

/// Implementation of [`okf::vnode::VopRead`] for 11.00.
#[repr(C)]
pub struct VopRead {
    desc: *mut VnodeOp,
    vp: *mut Vnode,
    uio: *mut Uio,
    flags: c_int,
    cred: *mut Ucred,
}

impl okf::vnode::VopRead<crate::Kernel> for VopRead {
    unsafe fn new(
        k: crate::Kernel,
        vp: *mut Vnode,
        uio: *mut Uio,
        flags: c_int,
        cred: *mut Ucred,
    ) -> Self {
        Self {
            desc: k.var(crate::Kernel::VOP_READ).ptr(),
            vp,
            uio,
            flags,
            cred,
        }
    }
}

/// Implementation of [`okf::vnode::VopReadDir`] for 11.00.
#[repr(C)]
pub struct VopReadDir {
    desc: *mut VnodeOp,
    vp: *mut Vnode,
    uio: *mut Uio,
    cred: *mut Ucred,
    eof: *mut c_int,
    ncookies: *mut c_int,
    cookies: *mut *mut u64,
}

impl okf::vnode::VopReadDir<crate::Kernel> for VopReadDir {
    unsafe fn new(
        k: crate::Kernel,
        vp: *mut Vnode,
        uio: *mut Uio,
        cred: *mut Ucred,
        eof: *mut c_int,
        ncookies: *mut c_int,
        cookies: *mut *mut u64,
    ) -> Self {
        Self {
            desc: k.var(crate::Kernel::VOP_READDIR).ptr(),
            vp,
            uio,
            cred,
            eof,
            ncookies,
            cookies,
        }
    }
}
