mod ui;
use crate::{
    window_manager::MouseState,
    record_store::{
        Archetype,
        ActivityRecord,
        RecordTracker,
        RecordStore,
    }
};
use std::time;
use ui::*;

pub struct AppState {
    // Tracking information
    record_tracker: RecordTracker,
    last_moment_active: time::SystemTime,
    last_mouse_position: (i32, i32),
    last_active_window: Option<Archetype>,

    // TUI state
    pub router: Router,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            last_moment_active: time::SystemTime::now(),
            last_mouse_position: (0, 0),
            router: Router::default(),
            record_tracker: RecordTracker::new(),
            last_active_window: None,
        }
    }

    pub fn update_window_info(&mut self, info: Option<Archetype>) {
        let is_same_window = info == self.last_active_window;
        let is_afk = is_same_window && self.get_afk_seconds() > 10;
        let info_clone = info.clone();
        let mut record: Option<ActivityRecord> = None;

        if is_afk {
            record = self.record_tracker.ping(Some(Archetype::AFK));
        } else {
            record = self.record_tracker.ping(info);

            if !is_same_window {
                self.timer_reset();
                self.last_active_window = info_clone;
            }
        }

        /// pass the record over to record store
    }

    pub fn update_mouse_info(&mut self, mouse_info: &MouseState) {
        if self.last_mouse_position != mouse_info.coords {
            self.last_mouse_position = mouse_info.coords;
            self.timer_reset();
        }

        if mouse_info.button_pressed.contains(&true) {
            self.timer_reset()
        }
    }

    pub fn update_keys(&mut self, comb: Vec<u8>) {
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

    /// Gets current tracking information
    pub fn get_current_archetype(&self) -> &Option<Archetype> {
        self.record_tracker.get_current_archetype()
    }

    fn timer_reset(&mut self) {
        self.last_moment_active = time::SystemTime::now();
    }
}
