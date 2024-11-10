use crate::ucred::Ucred;
use crate::uio::Uio;
use crate::Kernel;
use core::ffi::c_int;

/// Implementation of [`okf::vnode::Vnode`] for 11.00.
#[repr(C)]
pub struct Vnode {
    pad1: [u8; 0x10],
    ops: *mut VopVector,
}

impl okf::vnode::Vnode<Kernel> for Vnode {
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

impl okf::vnode::VopRead<Kernel> for VopRead {
    unsafe fn new(
        k: Kernel,
        vp: *mut Vnode,
        uio: *mut Uio,
        flags: c_int,
        cred: *mut Ucred,
    ) -> Self {
        use okf::Kernel;

        Self {
            desc: k.var(crate::Kernel::VOP_READ).ptr(),
            vp,
            uio,
            flags,
            cred,
        }
    }
}
