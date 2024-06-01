use core::ffi::c_short;

/// Implementation of [`okf::socket::Socket`] for 11.00.
#[repr(C)]
pub struct Socket {
    pad1: [u8; 0x6d],
    timeout: c_short,
}

impl okf::socket::Socket for Socket {
    fn timeout_mut(&mut self) -> &mut c_short {
        &mut self.timeout
    }
}
