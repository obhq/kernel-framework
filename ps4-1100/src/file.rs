use core::sync::atomic::AtomicU32;

/// Implementation of [`okf::file::File`] for 11.00.
#[repr(C)]
pub struct File {
    pad: [u8; 0x28],
    refcnt: AtomicU32,
}

impl okf::file::File for File {
    fn refcnt(&self) -> &AtomicU32 {
        &self.refcnt
    }
}
