mod ui;
use crate::{
    window_manager::MouseState,
    record_store::{
        Archetype,
        RecordTracker,
        RecordStore
    }
};
use std::time;
use ui::*;

pub struct AppState {
    // Tracking information
    pub active_window_info: Option<Archetype>,
    last_moment_active: time::SystemTime,
    last_mouse_position: (i32, i32),

    // TUI state
    pub router: Router,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_window_info: None,
            last_moment_active: time::SystemTime::now(),
            last_mouse_position: (0, 0),
            router: Router::default(),
        }
    }

    pub fn updated_window_info(&mut self, info: Option<Archetype>) {
        let current_info = &self.active_window_info;
        
        if match (current_info, &info) {
            (Some(current_arch), Some(in_arch)) => current_arch != in_arch,
            (None, None) => false,
            _ => true
        } {
            self.timer_reset();
            self.active_window_info = info;
        }
    }

    pub fn updated_mouse_info(&mut self, mouse_info: &MouseState) {
        if self.last_mouse_position != mouse_info.coords {
            self.last_mouse_position = mouse_info.coords;
            self.timer_reset();
        }

        if mouse_info.button_pressed.contains(&true) {
            self.timer_reset()
        }
    }

    pub fn updated_keys(&mut self, comb: Vec<u8>) {
        if comb.len() > 0 {
            self.timer_reset();
        }
    }

    pub fn get_afk_seconds(&self) -> u64 {
        time::SystemTime::now()
            .duration_since(self.last_moment_active)
            .unwrap_or(time::Duration::new(0, 0))
            .as_secs()
    }

    fn timer_reset(&mut self) {
        self.last_moment_active = time::SystemTime::now();
    }
}
