use crate::fd::{openat, write_all, OpenFlags, AT_FDCWD};
use crate::pcpu::Pcpu;
use crate::uio::UioSeg;
use crate::Kernel;
use core::ffi::c_int;
use core::fmt::Write;

/// Notification to send to the PlayStation 4 shell.
///
/// Use [write](core::write) macro to write the message.
///
/// [`Write`] implementation on this type never return [`Err`]. Instead, it will discard all writes
/// after the buffer is full.
pub struct Notification {
    data: OrbisNotificationRequest,
    next: usize,
}

impl Notification {
    pub fn new() -> Self {
        Self {
            data: OrbisNotificationRequest {
                ty: 0,
                req_id: 0,
                priority: 0,
                msg_id: 0,
                target_id: -1,
                user_id: 0,
                unk1: 0,
                unk2: 0,
                app_id: 0,
                error_num: 0,
                unk3: 0,
                use_icon_image_uri: 1,
                message: [0; 1024],
                icon_uri: [0; 1024],
                unk: [0; 1024],
            },
            next: 0,
        }
    }

    #[inline(never)]
    pub fn send<K: Kernel>(self, k: K) {
        // Open notification device.
        let devs = [c"/dev/notification0", c"/dev/notification1"];
        let mut fd = None;

        for dev in devs.into_iter().map(|v| v.as_ptr()) {
            if let Ok(v) =
                unsafe { openat(k, AT_FDCWD, dev, UioSeg::Kernel, OpenFlags::O_WRONLY, 0) }
            {
                fd = Some(v);
                break;
            }
        }

        // Check if we have a device to write to.
        let fd = match fd {
            Some(v) => v,
            None => return,
        };

        // Write notification.
        let len = size_of_val(&self.data);
        let data = &self.data as *const OrbisNotificationRequest as *const u8;
        let data = unsafe { core::slice::from_raw_parts(data, len) };
        let td = K::Pcpu::curthread();

        unsafe { write_all(k, fd.as_raw_fd(), data, td).ok() };
    }
}

impl Default for Notification {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for Notification {
    #[inline(never)]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Get end offset.
        let end = match self.next.checked_add(s.len()) {
            Some(v) => v,
            None => {
                // Instead of error we discard all subsequence write.
                self.next = self.data.message.len();
                return Ok(());
            }
        };

        // Get location to write to.
        let dst = match self.data.message.get_mut(self.next..end) {
            Some(v) => v,
            None => {
                // Same as the above.
                self.next = self.data.message.len();
                return Ok(());
            }
        };

        dst.copy_from_slice(s.as_bytes());
        self.next = end;

        Ok(())
    }

    #[inline(never)]
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }
}

/// By OSM-Made.
#[repr(C)]
struct OrbisNotificationRequest {
    ty: c_int,
    req_id: c_int,
    priority: c_int,
    msg_id: c_int,
    target_id: c_int,
    user_id: c_int,
    unk1: c_int,
    unk2: c_int,
    app_id: c_int,
    error_num: c_int,
    unk3: c_int,
    use_icon_image_uri: u8,
    message: [u8; 1024],
    icon_uri: [u8; 1024],
    unk: [u8; 1024],
}
