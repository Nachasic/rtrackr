use x11::xlib::{
    Atom as XAtom,
    True as XTrue,
    XInternAtom,
};
use std::{
    os::raw::{
        c_ulong
    },
    ffi::{
        CString,
        NulError,
    }
};
use crate::{ Display, Null };

pub enum AtomType {
    WM_NAME,
    WM_WINDOW_ROLE,
    WM_CLASS,
    NET_WM_PID,
    NET_WM_NAME,
    NET_CLIENT_LIST,
    NET_ACTIVE_WINDOW
}

/// A wrapper around a [x11::xlib::Atom].
/// 
/// See the [Atom::new] function for an example on how to create one.
#[derive(Copy, Clone, Debug)]
pub struct Atom(pub XAtom);
impl Atom {
    /// An export of [XInternAtom] that turns a [CString] into a Atom.
    /// 
    /// An Error is only created if the [CString] has a null byte in it.
    /// If it does a [NulError] is returned.
    /// 
    /// # Example
    /// ```ignore
    /// Atom::new(AtomType::WM_WINDOW_ROLE)
    ///     .expect("Could not create the CString");
    /// ```
    pub fn new (display: &Display, atom_type: AtomType) -> Result<Self, NulError> {
        let text = Self::get_c_string(atom_type)?;
        let atom = unsafe { XInternAtom(display.0, text.as_ptr(), XTrue) };
        Ok(Atom(atom))
    }

    pub fn get_c_string (atom_type: AtomType) -> Result<CString, NulError> {
        match atom_type {
            AtomType::WM_CLASS => Ok(CString::new("WM_CLASS")?),
            AtomType::WM_NAME => Ok(CString::new("WM_NAME")?),
            AtomType::WM_WINDOW_ROLE => Ok(CString::new("WM_WINDOW_ROLE")?),
            AtomType::NET_WM_PID => Ok(CString::new("_NET_WM_PID")?),
            AtomType::NET_WM_NAME => Ok(CString::new("_NET_WM_NAME")?),
            AtomType::NET_CLIENT_LIST => Ok(CString::new("_NET_CLIENT_LIST")?),
            AtomType::NET_ACTIVE_WINDOW => Ok(CString::new("_NET_ACTIVE_WINDOW")?)
        }
    }
}