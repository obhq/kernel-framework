use core::ffi::c_char;

/// Represents `dirent` structure.
#[repr(C)]
pub struct DirEnt<const L: usize> {
    pub id: u32,           // d_fileno
    pub len: u16,          // d_reclen
    pub ty: u8,            // d_type
    pub name_len: u8,      // d_namlen
    pub name: [c_char; L], // d_name
}
