/// Represents a struct produced by `TAILQ_HEAD`.
#[repr(C)]
pub struct TailQueue<T> {
    pub first: *mut T,     // tqh_first
    pub last: *mut *mut T, // tqh_last
}

/// Represents a struct produced by `TAILQ_ENTRY`.
#[repr(C)]
pub struct TailQueueEntry<T> {
    pub next: *mut T,      // tqe_next
    pub prev: *mut *mut T, // tqe_prev
}
