/// Represents `sockaddr_in` structure.
#[repr(C)]
pub struct SockAddrIn {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

/// Represents `in_addr` structure.
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}
