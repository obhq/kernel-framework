use crate::Kernel;

/// Implementation of [`okf::pcpu::Pcpu`] for 11.00.
#[repr(C)]
pub struct Pcpu {}

impl okf::pcpu::Pcpu<Kernel> for Pcpu {}
