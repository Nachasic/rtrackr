use x11::xlib::{
    Atom as XAtom,
    False as xFalse,
    True as XTrue,
    XInternAtom,
    XGetWindowProperty,
    XFree,

};
use std::{
    error::{ Error },
    fmt,
    os::raw::{
        c_ulong,
        c_void,
        c_uchar,
        c_int
    },
    ptr::null_mut,
    slice,
    ffi::{
        CString,
    }
};
use crate::*;

#[derive(Debug)]
pub enum XAtomError {
    NoProperty(String),
    FailedCString(std::str::Utf8Error)
}

impl fmt::Display for XAtomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XAtomError::FailedCString(ref err) => write!(f, "Failed to parse CString {}", err),
            XAtomError::NoProperty(name) => write!(f, "Failed to retrieve property {} for a window", name)
        }
    }
}

impl Error for XAtomError {
    fn description(&self) -> &str {
        match self {
            XAtomError::FailedCString(_) => "failed to parse CString",
            XAtomError::NoProperty(_) => "failed to retrieve property WM_NAME for a window"
        }
    }

    fn cause(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            XAtomError::FailedCString(ref err) => Some(err),
            XAtomError::NoProperty(_) => None
        }
    }
}

impl From<std::str::Utf8Error> for XAtomError {
    fn from(err: std::str::Utf8Error) -> Self {
        XAtomError::FailedCString(err)
    }
}

/// Trait used to get raw information from the atom via ffi
///
/// `fn get_name` and `fn get_expected_property_type` are required for implementation
pub trait RawAtom<'a> {
    fn get_name() -> &'a str;
    fn get_expected_property_type(&self) -> XAtom;
    fn get(display: &Display) -> XAtom {
        unsafe { XInternAtom(
            display.0, 
            CString::new(
                XNetActiveWindow::get_name()
            ).unwrap().as_ptr(), 
            XTrue)
        }
    }
    fn get_as_raw_property(&self, display: &Display, window: &Window) -> Result<usize, XAtomError> {
        let mut actual_type_return: XAtom = 0;
        let mut actual_format_return: c_int = 0;
        let mut num_items_return: c_ulong = 0;
        let mut bytes_after_return: c_ulong = 0;
        let mut proper_return: *mut c_uchar = null_mut();

        if unsafe { XGetWindowProperty(
            display.0,
            window.0, 
            Self::get(&display),
            0, 4096 / 4,
            xFalse, 
            self.get_expected_property_type(),
            &mut actual_type_return,
            &mut actual_format_return,
            &mut num_items_return,
            &mut bytes_after_return,
            &mut proper_return
        ) } == 0 {
            let value = match actual_format_return {
                8 => {
                    unsafe { slice::from_raw_parts(proper_return as *const u8, num_items_return as usize) }
                        .first()
                        .map(|x| { *x as usize })
                },
                16 => {
                    unsafe { slice::from_raw_parts(proper_return as *const u16, num_items_return as usize) }
                        .first()
                        .map(|x| { *x as usize })
                },
                32 => {
                    unsafe { slice::from_raw_parts(proper_return as *const usize, num_items_return as usize) }
                        .first()
                        .map(|x| { *x })
                },
                _ => { return Err(XAtomError::NoProperty(
                    Self::get_name().to_owned()
                )) },
            };
            unsafe { XFree(proper_return as *mut c_void) };

            match value {
                None => return Err(XAtomError::NoProperty(
                    Self::get_name().to_owned()
                )),
                Some(val) => return Ok(val)
            };
        }
        Err(XAtomError::NoProperty(
            Self::get_name().to_owned()
        ))
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
    fn get_as_property(&self, display: &Display, window: &Window) -> Result<Self::PropertyType, Self::ErrorType>;
}
