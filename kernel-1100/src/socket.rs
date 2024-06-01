use core::ffi::{c_short, c_ushort};

/// Implementation of [`okf::socket::Socket`] for 11.00.
#[repr(C)]
pub struct Socket {
    pad1: [u8; 0x6d],
    timeout: c_short,
    error: c_ushort,
}

impl okf::socket::Socket for Socket {
    fn error(&self) -> c_ushort {
        self.error
    }

    fn timeout_mut(&mut self) -> &mut c_short {
        &mut self.timeout
    }
}
