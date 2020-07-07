/// TODO: Find out about "Bad Window" response when blocking the screen wihile tracking is enabled
mod atom;
mod atoms;
mod display;
mod events;

use crate::{
    window_manager::{MouseState, OSWindowManager},
    record_store::Archetype,
};
use atom::*;
use atoms::*;
use display::Display;
use events::*;

pub struct XORGWindowManager {
    display: Display,
    root_window: u64,
}

impl Default for XORGWindowManager {
    fn default() -> Self {
        let display = Display::open().unwrap();
        let root_window = display.get_default_root_window();
        Self {
            display,
            root_window,
        }
    }
}

impl XORGWindowManager {
    pub fn get_window_archetype(&self) -> Result<Archetype, Box<dyn std::error::Error>> {
        let active_window_uid = XNetActiveWindow::get_as_property(&self.display, self.root_window)?;
        let title = XWMName::get_as_property(&self.display, active_window_uid)?;
        let (app_name, app_class) = XWMClass::get_as_property(&self.display, active_window_uid)?;


        Ok(Archetype::ActiveWindow(title, app_name, app_class))
    }
}

impl OSWindowManager for XORGWindowManager {
    type KeyboardState = Vec<u8>;

    fn get_window_archetype(&self) -> Option<Archetype> {
        match self.get_window_archetype() {
            Ok(arch) => Some(arch),
            Err(_) => None
        }
    }

    fn query_keyboard(&self) -> Self::KeyboardState {
        query_keyboard(&self.display)
    }

    fn query_mouse_pointer(&self) -> MouseState {
        query_mouse_pointer(&self.display, self.root_window)
    }
}
