use x11::xlib;

use std::ffi::CStr;
use std::ptr;
use std::mem;

pub struct Display {
    dpy: *mut xlib::_XDisplay,
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.dpy);
        }
    }
}

impl Display {
    pub fn open() -> Result<Display, Box<dyn std::error::Error>> {
        unsafe {
            let dpy = xlib::XOpenDisplay(ptr::null());

            if dpy.is_null() {
                Err("xlib::XOpenDisplay returned null".into())
            } else {
                Ok(Display {
                    dpy,
                })
            }
        }
    }

    pub fn grab_input(&mut self) {
        unsafe {
            let mut focused = 0u64;
            let mut revert_to = 0i32;

            xlib::XGetInputFocus(self.dpy, &mut focused, &mut revert_to);
            xlib::XSelectInput(self.dpy, focused, xlib::KeyPressMask);
        }
    }

    pub fn poll_event(&mut self) -> Option<xlib::XEvent> {
        unsafe {
            if xlib::XPending(self.dpy) > 0 {
                let mut event: xlib::XEvent = mem::zeroed();
                xlib::XNextEvent(self.dpy, &mut event);

                Some(event)
            } else {
                None
            }
        }
    }

    pub fn keycode_to_string(&mut self, keycode: u8) -> &str {
        unsafe {
            CStr::from_ptr(xlib::XKeysymToString(xlib::XKeycodeToKeysym(self.dpy, keycode, 0))).to_str().unwrap_or_default()
        }
    }
}

