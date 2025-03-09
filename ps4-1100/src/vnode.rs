use crate::namei::ComponentName;
use crate::ucred::Ucred;
use crate::uio::Uio;
use core::ffi::c_int;
use okf::Kernel;

/// Implementation of [`okf::vnode::Vnode`] for 11.00.
#[repr(C)]
pub struct Vnode {
    ty: c_int,
    pad1: [u8; 0xC],
    ops: *mut VopVector,
}

impl okf::vnode::Vnode<crate::Kernel> for Vnode {
    fn ty(&self) -> c_int {
        self.ty
    }

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
            desc: k.get(crate::Kernel::VOP_READ).as_mut_ptr(),
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
            desc: k.get(crate::Kernel::VOP_READDIR).as_mut_ptr(),
            vp,
            uio,
            cred,
            eof,
            ncookies,
            cookies,
        }
    }
}

/// Implementation of [`okf::vnode::VopLookup`] for 11.00.
#[repr(C)]
pub struct VopLookup {
    desc: *mut VnodeOp,
    vp: *mut Vnode,
    out: *mut *mut Vnode,
    cn: *mut ComponentName,
}

impl okf::vnode::VopLookup<crate::Kernel> for VopLookup {
    unsafe fn new(
        k: crate::Kernel,
        vp: *mut Vnode,
        out: *mut *mut Vnode,
        cn: *mut ComponentName,
    ) -> Self {
        Self {
            desc: k.get(crate::Kernel::VOP_LOOKUP).as_mut_ptr(),
            vp,
            out,
            cn,
        }
    }
}
