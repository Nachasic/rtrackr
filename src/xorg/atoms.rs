use x11::xlib::{
    Window as XWindow,
    Atom as XAtom,
    XA_WINDOW,
    XTextProperty,
    XClassHint,
    XGetWMName,
    XGetClassHint,
};
use std::{
    ptr::null_mut,
    ffi::CStr,
};
use crate::*;

/// Atom that corresponds with current active window under
/// root window on a given display
#[derive(Debug, Copy, Clone)]
pub struct XNetActiveWindow;
impl <'a> RawAtom <'a> for XNetActiveWindow {
    fn get_name() -> &'a str {
        return "_NET_ACTIVE_WINDOW"
    }
    fn get_expected_property_type(&self) -> XAtom {
        XA_WINDOW
    }
}
impl Atom for XNetActiveWindow {
    type PropertyType = Window;
    type ErrorType = XAtomError;

    /// Gets an active window object under a root window in a given display
    ///
    /// # Arguments
    /// * `display` - an object for a display
    /// * `window` - a parent window that contains the active window we're looking for
    ///
    /// # Example
    /// ```
    /// let Session { ref display, ref mut root_window, .. } = Session::open()?;
    /// let root_window = root_window.get_or_insert(Window::default_root_window(&display));
    /// let active_window: Window = XNetActiveWindow.get_as_property(display, root_window).unwrap();
    /// ```
    fn get_as_property(&self, display: &Display, window: &Window) -> Result<Self::PropertyType, Self::ErrorType> { 
        RawAtom::get_as_raw_property(self, display, window).map(
            |res| Window (res as XWindow)
        )
    }
}

/// Atom for retrieving a name for a given window
/// On a given display
#[derive(Debug, Copy, Clone)]
pub struct XWMName;
impl Atom for XWMName {
    type PropertyType = String;
    type ErrorType = XAtomError;

    fn get_as_property(&self, display: &Display, window: &Window) -> Result<Self::PropertyType, Self::ErrorType> {
        let mut text_property = XTextProperty {
            value: null_mut(),
            encoding: 0,
            format: 0,
            nitems: 0,
        };
        unsafe { 
            XGetWMName(
                display.0,
                window.0,
                &mut text_property,
            )
        };
        if !text_property.value.is_null() {
            let text = unsafe { CStr::from_ptr(text_property.value as *mut i8) };

            match text.to_str() {
                Ok(slice) => Ok(String::from(slice)),
                Err(err) => Err(XAtomError::from(err))
            }
        } else {
            Err(XAtomError::NoProperty(String::from("WM_NAME")))
        }
    }
}

/// Atom for retrieving class of a given window
/// on a given display
#[derive(Debug, Copy, Clone)]
pub struct XWMClass;
impl Atom for XWMClass {
    type PropertyType = (String, String);
    type ErrorType = XAtomError;

    fn get_as_property(&self, display: &Display, window: &Window) -> Result<Self::PropertyType, Self::ErrorType> {
        let mut class_hint = XClassHint {
            res_class: null_mut(),
            res_name: null_mut(),
        };

        unsafe {
            XGetClassHint(
                display.0,
                window.0,
                &mut class_hint
            )
        };

        if !class_hint.res_name.is_null() && !class_hint.res_class.is_null() {
            let name_text = unsafe { CStr::from_ptr(class_hint.res_name as *mut i8) };
            let class_text = unsafe { CStr::from_ptr(class_hint.res_class as *mut i8) };

            Ok((
                String::from(name_text.to_str()?),
                String::from(class_text.to_str()?)
            ))
        } else {
            Err(XAtomError::NoProperty(String::from("WM_CLASS")))
        }
    }
}