use super::display::*;
use std::{
    error::Error,
    ffi::CString,
    fmt,
    os::raw::{c_int, c_uchar, c_ulong, c_void},
    ptr::null_mut,
    slice,
    panic::catch_unwind,
};
use x11::xlib::{
    Atom as XAtom, False as xFalse, True as XTrue, Window, XFree, XGetWindowProperty, XInternAtom,
};

#[derive(Debug)]
pub enum XAtomError<'a> {
    NoProperty(&'a str),
    FailedCString(std::str::Utf8Error),
    BadWindow,
}

impl<'a> fmt::Display for XAtomError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XAtomError::FailedCString(ref err) => write!(f, "Failed to parse CString {}", err),
            XAtomError::NoProperty(name) => write!(f, "Failed to retrieve property {} for a window", name),
            XAtomError::BadWindow => write!(f, "XORG has risen BAD WINDOW"),
        }
    }
}

impl<'a> Error for XAtomError<'a> {
    fn description(&self) -> &str {
        match self {
            XAtomError::FailedCString(_) => "failed to parse CString",
            XAtomError::NoProperty(_) => "failed to retrieve property WM_NAME for a window",
            XAtomError::BadWindow => "XORG has risen BAD WINDOW",
        }
    }

    fn cause(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            XAtomError::FailedCString(ref err) => Some(err),
            XAtomError::NoProperty(_) => None,
            XAtomError::BadWindow => None
        }
    }
}

impl<'a> From<std::str::Utf8Error> for XAtomError<'a> {
    fn from(err: std::str::Utf8Error) -> Self {
        XAtomError::FailedCString(err)
    }
}

/// Trait used to get raw information from the atom via ffi
///
/// `fn get_name` and `fn get_expected_property_type` are required for implementation
pub trait RawAtom<'a> {
    fn get_name() -> &'a str;
    fn get_expected_property_type() -> XAtom;
    fn get(display: &Display) -> XAtom {
        unsafe {
            XInternAtom(
                display.0,
                CString::new(Self::get_name()).unwrap().as_ptr(),
                XTrue,
            )
        }
    }
    fn get_as_raw_property(display: &Display, window: Window) -> Result<usize, XAtomError<'a>> {
        catch_unwind(|| {
            let mut actual_type_return: XAtom = 0;
            let mut actual_format_return: c_int = 0;
            let mut num_items_return: c_ulong = 0;
            let mut bytes_after_return: c_ulong = 0;
            let mut proper_return: *mut c_uchar = null_mut();

            if unsafe {
                XGetWindowProperty(
                    display.0,
                    window,
                    Self::get(&display),
                    0,
                    4096 / 4,
                    xFalse,
                    Self::get_expected_property_type(),
                    &mut actual_type_return,
                    &mut actual_format_return,
                    &mut num_items_return,
                    &mut bytes_after_return,
                    &mut proper_return,
                )
            } == 0
            {
                let value = match actual_format_return {
                    8 => unsafe {
                        slice::from_raw_parts(proper_return as *const u8, num_items_return as usize)
                    }
                    .first()
                    .map(|x| *x as usize),
                    16 => unsafe {
                        slice::from_raw_parts(proper_return as *const u16, num_items_return as usize)
                    }
                    .first()
                    .map(|x| *x as usize),
                    32 => unsafe {
                        slice::from_raw_parts(proper_return as *const usize, num_items_return as usize)
                    }
                    .first()
                    .map(|x| *x),
                    _ => return Err(XAtomError::NoProperty(Self::get_name())),
                };
                unsafe { XFree(proper_return as *mut c_void) };

                match value {
                    None => return Err(XAtomError::NoProperty(Self::get_name())),
                    Some(val) => return Ok(val),
                };
            }
            return Err(XAtomError::NoProperty(Self::get_name()))
        }).map_err(|_| XAtomError::BadWindow)?
    }
}

/// Trait used to convert raw property gotten from ffi
/// to any internally used data structure
pub trait Atom {
    type PropertyType;
    type ErrorType;

    /// Gets window's property via this atom and converts the result
    /// to an internally used format.
    ///
    /// Most implementations call
    /// `RawAtom::get_as_raw_property(self, display, window)` internally to then
    /// convert raw results to `PropertyType`
    fn get_as_property(
        display: &Display,
        window: Window,
    ) -> Result<Self::PropertyType, Self::ErrorType>;
}
