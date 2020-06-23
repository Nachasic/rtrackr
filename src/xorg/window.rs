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
    XWMName,
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
    pub fn active_window(session: &mut Session) -> 
        Result<Window, <XNetActiveWindow as Atom>::ErrorType> {
            let Session { ref display, ref mut root_window, .. } = session;
            let root_window = root_window.get_or_insert(Window::default_root_window(&display));
            XNetActiveWindow.get_as_property(display, &root_window)
        }

    /// Gets the title of the window.
    pub fn get_title(self, display: &Display) ->
        Result<String, <XWMName as Atom>::ErrorType> {
        XWMName.get_as_property(display, &self)
    }

    // pub fn get_roles(self, display: &Display) -> Result<Vec<String>, Null> {
    //     let mut 
    // }
}