use x11::xlib::{
    Window as XWindow,
    XDefaultRootWindow,
    XGetWMName,
    XTextProperty,
};
use std::{
    ffi::CStr,
    ptr::null_mut,
};
use crate::{
    Display,
    Session,
    XNetActiveWindow,
    Atom,
    Null,
};

/// This struct represents a window and holds the ID of that window that can be used
/// to query for its name.
#[derive(Copy, Clone, Debug)]
pub struct Window(pub XWindow);
impl Window {
    /// Gets the default root window of a display.
    /// 
    /// A wrapper around the [XDefaultRootWindow] function.
    pub fn default_root_window(display: &Display) -> Self {
        let win = unsafe { XDefaultRootWindow(display.0) };
        Window(win)
    }

    /// Gets the current active window.
    /// 
    /// This function uses a [Session] struct and will update the properties
    /// that are set to [None] but are required.
    /// This uses the display, root_window, and active_window_atom properties
    /// of the [Session] struct.
    pub fn active_window(session: &mut Session) -> Option<Self> {
        let Session { ref display, ref mut root_window, .. } = session;
        let root_window = root_window.get_or_insert(Window::default_root_window(&display));
        XNetActiveWindow.get_as_property(display, &root_window)
    }

    /// Gets the title of the window.
    pub fn get_title(self, display: &Display) -> Result<String, Null> {
        let mut text_property = XTextProperty {
            value: null_mut(),
            encoding: 0,
            format: 0,
            nitems: 0,
        };
        unsafe { 
            XGetWMName(
                display.0,
                self.0,
                &mut text_property,
            )
        };
        if !text_property.value.is_null() {
            let text = unsafe { CStr::from_ptr(text_property.value as *mut i8) };
            text.to_str().map_or(
                Err(Null),
                |slice| Ok(String::from(slice)))
        } else { Err(Null) }
    }
}