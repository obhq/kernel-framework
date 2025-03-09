use core::cmp::min;
use core::ffi::c_char;
use core::fmt::Write;

/// Provides [`Write`] implementation backed by a fixed size buffer.
pub struct Message {
    buf: [u8; 2048],
    pos: usize,
}

impl Message {
    pub fn as_ptr(&self) -> *const c_char {
        self.buf.as_ptr().cast()
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            buf: [0; 2048],
            pos: 0,
        }
    }
}

impl Write for Message {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // This method need to be careful not to cause any panic so we don't end up nested panic.
        let len = min(s.len(), (self.buf.len() - 1) - self.pos);
        let buf = unsafe { self.buf.as_mut_ptr().add(self.pos) };

        unsafe { buf.copy_from_nonoverlapping(s.as_ptr(), len) };
        self.pos += len;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_str() {
        let mut m = Message::default();

        write!(m, "Hello, world!").unwrap();

        assert!(m.buf.as_slice().starts_with(b"Hello, world!\0"));

        // Make sure repeated write on a full buffer won't do anything.
        for _ in 0..m.buf.len() {
            write!(m, "Hello, world!").unwrap();
        }

        assert!(m.buf.as_slice().starts_with(b"Hello, world!Hello"));
        assert_eq!(m.buf.last().copied().unwrap(), 0);
    }
}
