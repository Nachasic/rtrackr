mod xorg;
use x11::xlib::{
    Atom as XAtom,
    False as xFalse,
    True as XTrue,
    Window as XWindow,
    XInternAtom,
    XGetWindowProperty,
    XFree,
    XA_WINDOW
};
use std::{
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
        NulError,
    }
};

use xorg::*;

pub trait RawAtom {
    fn get (display: &Display) -> XAtom;
    fn get_expected_property_type (&self) -> XAtom;
    fn get_as_raw_property (&self, display: &Display, window: &Window) -> Option<usize> {
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
                _ => { None },
            };
            unsafe { XFree(proper_return as *mut c_void) };
            return value
        }
        None
    }
}

pub trait Atom: RawAtom {
    type PropertyType;
    fn get_as_property (&self, display: &Display, window: &Window) -> Option<Self::PropertyType>;
}

#[derive(Debug, Copy, Clone)]
struct XNetActiveWindow;
impl RawAtom for XNetActiveWindow {
    fn get(display: &Display) -> XAtom {
        unsafe { XInternAtom(
            display.0, 
            CString::new("_NET_ACTIVE_WINDOW").unwrap().as_ptr(), 
            XTrue)
        }
    }
    fn get_expected_property_type(&self) -> XAtom {
        XA_WINDOW
    }
}
impl Atom for XNetActiveWindow {
    type PropertyType = Window;
    fn get_as_property(&self, display: &Display, window: &Window) -> Option<Self::PropertyType> { 
        RawAtom::get_as_raw_property(self, display, window).map(
            |res| Window (res as XWindow)
        )
    }
}

fn main () -> Result<(), Null> {
    let Session { ref display, ref mut root_window, .. } = Session::open()?;
    let root_window = root_window.get_or_insert(Window::default_root_window(&display));
    let active_window = XNetActiveWindow.get_as_property(display, root_window).unwrap();

    let title = active_window.get_title(&display)?;
    println!("{:?}", title);
    Ok({})
}