use x11::xlib::{
    Display as XDisplay,
    XOpenDisplay,
    XCloseDisplay
};
use std::{
    ptr::null,
    ops::Drop
};

// TODO better error handling
#[derive(Debug)]
pub struct Null;

/// The Display Struct is just a wrapper of a [*mut Display] from XLib.
/// 
/// When this struct is dropped, the reference will be dropped using [XCloseDisplay].
pub struct Display(pub(crate) *mut XDisplay);
impl Display {
    /// Opens a connection to the x11 server.
    /// 
    /// Will return an error of [Null] if the returned Display pointer is a null pointer.
    pub fn open() -> Result<Self, Null> {
        let x_display = unsafe { XOpenDisplay( null() ) };
        if x_display.is_null() {
            return Err(Null)
        }
        Ok(Display(x_display))
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { XCloseDisplay(self.0) };
    }
}