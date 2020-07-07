use super::{atom::*, display::*};
use std::{
    ffi::{ CStr, c_void },
    marker::PhantomData,
    ptr::null_mut,
    panic::catch_unwind,
    ops::Drop,
};
use x11::xlib::{
    Atom as XAtom, Window, XClassHint, XGetClassHint, XGetWMName, XTextProperty, XA_WINDOW, XFree
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
#[derive(Debug, Clone)]
pub struct XWMName<'a> {
    phantom: PhantomData<&'a str>,
    pub property: XTextProperty,
}

impl <'a> Default for XWMName<'a> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
            property: XTextProperty {
                value: null_mut(),
                encoding: 0,
                format: 0,
                nitems: 0,
            }
        }
    }
}

impl <'a> Drop for XWMName<'a> {
    fn drop(&mut self) {
        let value = self.property.value;
        unsafe { XFree(value as *mut c_void) };
    }
}

impl<'a> Atom for XWMName<'a> {
    type PropertyType = String;
    type ErrorType = XAtomError<'a>;

    fn get_as_property(
        display: &Display,
        window: Window,
    ) -> Result<Self::PropertyType, Self::ErrorType> {
        catch_unwind(|| {
            let mut atom = Self::default();
            unsafe { XGetWMName(display.0, window, &mut atom.property) };

            if !atom.property.value.is_null() {
                let text = unsafe { CStr::from_ptr(atom.property.value as *mut i8) };
    
                return Ok(text.to_string_lossy().into_owned());
            } else {
                return Err(XAtomError::NoProperty("WM_NAME"));
            };
        }).map_err(|_| XAtomError::BadWindow)?
    }
}

/// Atom for retrieving class of a given window
/// on a given display
#[derive(Debug, Clone)]
pub struct XWMClass<'a> {
    phantom: PhantomData<&'a str>,
    property: XClassHint
}

impl <'a> Default for XWMClass<'a> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
            property: XClassHint {
                res_class: null_mut(),
                res_name: null_mut(),
            }
        }
    }
}

impl <'a> Drop for XWMClass<'a> {
    fn drop(&mut self) {
        let res_class = self.property.res_class;
        let res_name = self.property.res_name;

        unsafe {
            XFree(res_class as *mut c_void);
            XFree(res_name as *mut c_void);
        };
    }
}

impl<'a> Atom for XWMClass<'a> {
    type PropertyType = (String, String);
    type ErrorType = XAtomError<'a>;

    fn get_as_property(
        display: &Display,
        window: Window,
    ) -> Result<Self::PropertyType, Self::ErrorType> {
        catch_unwind(|| {
            let mut atom = Self::default();
            unsafe { XGetClassHint(display.0, window, &mut atom.property) };
    
            if !atom.property.res_name.is_null() && !atom.property.res_class.is_null() {
                let name_text = unsafe { CStr::from_ptr(atom.property.res_name as *mut i8) };
                let class_text = unsafe { CStr::from_ptr(atom.property.res_class as *mut i8) };
    
                return Ok((
                    String::from(name_text.to_str()?),
                    String::from(class_text.to_str()?),
                ))
            } else {
                return Err(XAtomError::NoProperty("WM_CLASS"))
            }
        }).map_err(|_| XAtomError::BadWindow)?
    }
}
