use okf::queue::TailQueueEntry;

/// Implementation of [`okf::mount::Mount`] for 11.00.
#[repr(C)]
pub struct Mount {
    pad1: [u8; 0x28],
    entry: TailQueueEntry<Self>,
}

impl okf::mount::Mount for Mount {
    fn entry(&self) -> &TailQueueEntry<Self> {
        &self.entry
    }

    fn entry_mut(&mut self) -> &mut TailQueueEntry<Self> {
        &mut self.entry
    }
}
