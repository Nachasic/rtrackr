use x11::xlib::{
    Window as XWindow,
};

/// This struct represents a window and holds the ID of that window
#[derive(Copy, Clone, Debug)]
pub struct Window(pub XWindow);