use crate::{ 
    record_store::{ Archetype }
};

pub struct MouseState {
    pub coords: (i32, i32),
    pub button_pressed: Vec<bool>,
}

pub trait OSWindowManager {
    type KeyboardState;

    fn get_window_archetype(&self) -> Option<Archetype>;
    fn query_mouse_pointer(&self) -> MouseState;
    fn query_keyboard(&self) -> Self::KeyboardState;
}
