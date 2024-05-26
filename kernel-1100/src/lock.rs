/// Implementation of [`okf::lock::LockObject`] for 11.00.
#[repr(C)]
pub struct LockObject {}

impl okf::lock::LockObject for LockObject {}

/// Implementation of [`okf::lock::Mtx`] for 11.00.
#[repr(C)]
pub struct Mtx {}

impl okf::lock::Mtx for Mtx {}
