use x11::xlib::{Display as XDisplay, Window, XCloseDisplay, XDefaultRootWindow, XOpenDisplay};
// use crate::Window;
use std::{ops::Drop, ptr::null};

/// The Display Struct is just a wrapper of a [*mut Display] from XLib.
///
/// When this struct is dropped, the reference will be dropped using [XCloseDisplay].
#[derive(Debug)]
pub struct Display(pub *mut XDisplay);
impl Display {
    /// Opens a connection to the x11 server.
    ///
    /// Will return an error of [Null] if the returned Display pointer is a null pointer.
    pub fn open() -> Option<Self> {
        let x_display = unsafe { XOpenDisplay(null()) };
        if x_display.is_null() {
            return None;
        }
        Some(Display(x_display))
    }

    pub fn get_default_root_window(&self) -> Window {
        unsafe { XDefaultRootWindow(self.0) }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { XCloseDisplay(self.0) };
    }
}
