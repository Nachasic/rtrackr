use super::{atom::*, display::*};
use std::{ffi::CStr, marker::PhantomData, ptr::null_mut};
use x11::xlib::{
    Atom as XAtom, Window, XClassHint, XGetClassHint, XGetWMName, XTextProperty, XA_WINDOW,
};

/// Atom that corresponds with current active window under
/// root window on a given display
#[derive(Debug, Copy, Clone)]
pub struct XNetActiveWindow<'a> {
    phantom: PhantomData<&'a str>,
}
impl<'a> RawAtom<'a> for XNetActiveWindow<'a> {
    fn get_name() -> &'a str {
        return "_NET_ACTIVE_WINDOW";
    }
    fn get_expected_property_type() -> XAtom {
        XA_WINDOW
    }
}
impl<'a> Atom for XNetActiveWindow<'a> {
    type PropertyType = Window;
    type ErrorType = XAtomError<'a>;

    /// Gets an active window object under a root window in a given display
    ///
    /// # Arguments
    /// * `display` - an object for a display
    /// * `window` - a parent window that contains the active window we're looking for
    ///
    /// # Example
    /// ```
    /// let display = Display::open()?;
    /// let root_window = Window::default_root_window(&display);
    /// let active_window = XNetActiveWindow.get_as_property(&display, &root_window)?;
    /// ```
    fn get_as_property(
        display: &Display,
        window: Window,
    ) -> Result<Self::PropertyType, Self::ErrorType> {
        let raw_window = Self::get_as_raw_property(display, window)? as Window;
        Ok(raw_window)
    }
}

/// Atom for retrieving a name for a given window
/// On a given display
#[derive(Debug, Copy, Clone)]
pub struct XWMName<'a> {
    phantom: PhantomData<&'a str>,
}
impl<'a> Atom for XWMName<'a> {
    type PropertyType = String;
    type ErrorType = XAtomError<'a>;

    fn get_as_property(
        display: &Display,
        window: Window,
    ) -> Result<Self::PropertyType, Self::ErrorType> {
        let mut text_property = XTextProperty {
            value: null_mut(),
            encoding: 0,
            format: 0,
            nitems: 0,
        };
        unsafe { XGetWMName(display.0, window, &mut text_property) };
        if !text_property.value.is_null() {
            let text = unsafe { CStr::from_ptr(text_property.value as *mut i8) };

            Ok(text.to_string_lossy().into_owned())
        } else {
            Err(XAtomError::NoProperty("WM_NAME"))
        }
    }
}

/// Atom for retrieving class of a given window
/// on a given display
#[derive(Debug, Copy, Clone)]
pub struct XWMClass<'a> {
    phantom: PhantomData<&'a str>,
}
impl<'a> Atom for XWMClass<'a> {
    type PropertyType = (String, String);
    type ErrorType = XAtomError<'a>;

    fn get_as_property(
        display: &Display,
        window: Window,
    ) -> Result<Self::PropertyType, Self::ErrorType> {
        let mut class_hint = XClassHint {
            res_class: null_mut(),
            res_name: null_mut(),
        };

        unsafe { XGetClassHint(display.0, window, &mut class_hint) };

        if !class_hint.res_name.is_null() && !class_hint.res_class.is_null() {
            let name_text = unsafe { CStr::from_ptr(class_hint.res_name as *mut i8) };
            let class_text = unsafe { CStr::from_ptr(class_hint.res_class as *mut i8) };

            Ok((
                String::from(name_text.to_str()?),
                String::from(class_text.to_str()?),
            ))
        } else {
            Err(XAtomError::NoProperty("WM_CLASS"))
        }
    }
}
