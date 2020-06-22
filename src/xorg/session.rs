use x11::xlib::{
    Window as XWindow,
    XA_WINDOW,
    XFree,
};
use std::{
    os::raw::c_void,
    slice,
};
use crate::{
    Atom,
    AtomType,
    Display,
    Window,
    Null
};

/// This is purely for convenience.
/// 
/// # Example
/// ```ignore
/// let mut session = Session::open()
///    .expect("Error opening a new session.");
/// session
///    .get_windows()
///    .expect("Could not get a list of windows.")
///    .iter()
///    .filter_map(|x| x.get_title(&session.display).ok())
///    .for_each(|x| println!("{:?}", x.as_ref()))
/// // Prints out the title for every window that is visible on the screen.
/// ```
pub struct Session {
    /// A display that has been opened.
    pub display: Display,
    /// The root window of the display.
    pub root_window: Option<Window>,
    /// The atom that represents the client_list property.
    pub client_list_atom: Option<Atom>,
    /// The atom that represents the active_window property.
    pub active_window_atom: Option<Atom>,
}
impl Session {
    /// Opens a display.
    pub fn open() -> Result<Self, Null> {
        Ok( Self {
            display: Display::open()?,
            root_window: None,
            client_list_atom: None,
            active_window_atom: None,
        } )
    }
    /// Creates a session from an already opened Display connection.
    /// 
    /// See [Display::open] for more information.
    pub fn from_display(display: Display) -> Self {
        Self {
            display,
            root_window: None,
            client_list_atom: None,
            active_window_atom: None,
        }
    }
}
