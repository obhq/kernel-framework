use core::ffi::c_int;

/// Implementation of [`okf::vnode::Vnode`] for 11.00.
#[repr(C)]
pub struct Vnode {}

impl okf::vnode::Vnode for Vnode {}

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
