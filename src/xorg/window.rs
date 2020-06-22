use x11::xlib::{
    Window as XWindow,
    Atom as XAtom,
    XA_WINDOW,
    XDefaultRootWindow,
    XFree,
    XGetWMName,
    XTextProperty,
    XGetWindowProperty
};
use std::{
    ffi::CStr,
    ops::Drop,
    os::raw::c_void,
    ptr::null_mut,
    slice,
    os::raw::{
        c_int,
        c_uchar,
        c_ulong
    }
};
use crate::{
    RawAtom,
    Display,
    Null,
    Session,
    AtomType,
    // get_window_property
};

/// A response to [get_window_property].
/// 
/// A slice should be made from this.
/// 
/// **NOTE:** Remember to use XFree on the pointer.
/// 
/// # Example:
/// ```ignore
/// let response: GetWindowPropertyResponse = get_window_property(...);
/// if response.actual_format_return == 8 {
///     slice::from_raw_parts(response.proper_return as *const u8, response.nitems_return as usize)
///         .iter()
///         .for_each(|x| println!("{}", x));
///     XFree(response.proper_return)
/// }
/// ```
pub struct GetWindowPropertyResponse {
    /// The type of the return.
    pub actual_type_return: XAtom,
    /// The formate of the return whether it is 8, 16 or 32 bytes.
    /// If the architecture is 64-bits and the format is 32,
    /// then the return type wil be 64 bits.
    pub actual_format_return: c_int,
    /// The number of items returned in the lsit.
    pub nitems_return: c_ulong,
    /// The number of bytes that are returned.
    /// This crate ignores this field.
    pub bytes_after_return: c_ulong,
    /// The pointer that is returned.
    pub proper_return: *mut c_uchar,
}
impl Default for GetWindowPropertyResponse {
    fn default() -> Self {
        Self {
            actual_type_return: 0,
            actual_format_return: 0,
            nitems_return: 0,
            bytes_after_return: 0,
            proper_return: null_mut(),
        }
    }
}

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

    // pub fn get_window_property (self, display: &Display, atom: AtomType) -> Result<(), Null> {
    //     let response = GetWindowPropertyResponse::default();

    //     if XGetWindowProperty(
    //         display.0,
    //         window.0,
    //         property.0,
    //         0, 4096 / 4,
    //         XFalse,
    //         expected_type,
    //         &mut response.actual_type_return,
    //         &mut response.actual_format_return,
    //         &mut response.nitems_return,
    //         &mut response.bytes_after_return,
    //         &mut response.proper_return
    //     ) == 0 {
    //         return Ok(response)
    //     }
    //     Err(NotSupported)
    // }

    // /// Gets the current active window.
    // /// 
    // /// This function uses a [Session] struct and will update the properties
    // /// that are set to [None] but are required.
    // /// This uses the display, root_window, and active_window_atom properties
    // /// of the [Session] struct.
    // pub fn active_window(session: &mut Session) -> Option<Self> {
    //     let Session { display, root_window, active_window_atom, .. } = session;
    //     let root_window = root_window.get_or_insert(Window::default_root_window(display));
    //     let active_window_atom = active_window_atom.get_or_insert(
    //         Atom::new(&display, AtomType::NET_ACTIVE_WINDOW).expect("could not create atom")
    //     );
    //     let response = unsafe {
    //         let result = get_window_property(display, *root_window, *active_window_atom, XA_WINDOW);
    //         match result {
    //             Ok(res) => res,
    //             _ => panic!("could not get property")
    //         }
    //     };
    //     let window = match response.actual_format_return {
    //         8 => {
    //             unsafe{slice::from_raw_parts(response.proper_return as *const u8, response.nitems_return as usize)}
    //                 .first()
    //                 .map(|x| Window(*x as XWindow))
    //         },
    //         16 => {
    //             unsafe{slice::from_raw_parts(response.proper_return as *const u16, response.nitems_return as usize)}
    //                 .first()
    //                 .map(|x| Window(*x as XWindow))
    //         },
    //         32 => {
    //             unsafe{slice::from_raw_parts(response.proper_return as *const usize, response.nitems_return as usize)}
    //                 .first()
    //                 .map(|x| Window(*x as XWindow))
    //         },
    //         _ => { None },
    //     };
    //     unsafe{XFree(response.proper_return as *mut c_void)};
    //     window
    // }
    /// Gets the title of the window.
    /// 
    /// If the window does not have a title, a null pointer may be returned.
    /// In that case the [Null] error is returned.
    /// However, I have not encountered a [Null] error yet.
    pub fn get_title(self, display: &Display) -> Result<WindowTitle, Null> {
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
            Ok(WindowTitle(text))
        } else { Err(Null) }
    }
}

#[derive(Debug)]
pub struct WindowTitle<'a>(&'a CStr);
impl<'a> AsRef<CStr> for WindowTitle<'a> {
    fn as_ref(&self) -> &CStr {
        self.0
    }
}
impl<'a> Drop for WindowTitle<'a> {
    fn drop(&mut self) {
        unsafe { XFree(self.0.as_ptr() as *mut c_void) };
    }
}