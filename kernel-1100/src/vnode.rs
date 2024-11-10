use crate::Kernel;
use core::ffi::c_int;

/// Implementation of [`okf::vnode::Vnode`] for 11.00.
#[repr(C)]
pub struct Vnode {
    pad1: [u8; 0x10],
    ops: *mut VopVector,
}

impl okf::vnode::Vnode<Kernel> for Vnode {
    fn ops(&self) -> &'static VopVector {
        unsafe { &*self.ops }
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
