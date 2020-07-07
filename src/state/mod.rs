mod ui;
use crate::{
    window_manager::MouseState,
    record_store::{
        Archetype,
        RecordTracker,
        RecordStore,
        RecordStoreConfig
    },
    classifier::{
        Classifier, ClassifierConfig
    },
};
use std::time;
use ui::*;

pub struct AppState {
    // Tracking information
    last_moment_active: time::SystemTime,
    last_mouse_position: (i32, i32),
    last_active_window: Option<Archetype>,
    
    record_tracker: RecordTracker,
    record_store: RecordStore,
    record_classifier: Classifier,

    // TUI state
    pub router: Router,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            last_moment_active: time::SystemTime::now(),
            last_mouse_position: (0, 0),
            router: Router::default(),
            last_active_window: None,
            
            record_tracker: RecordTracker::new(),
            record_store: RecordStore::new(RecordStoreConfig::default())?,
            record_classifier: Classifier::from(ClassifierConfig::default())
        })
    }

    pub fn update_window_info(&mut self, info: Option<Archetype>) -> Result<(), Box<dyn std::error::Error>> {
        let is_same_window = info == self.last_active_window;
        let is_afk = is_same_window && self.get_afk_seconds() > 10;
        let info_clone = info.clone();

        let mut record = if is_afk {
            self.record_tracker.ping(Some(Archetype::AFK))
        } else { 
            if !is_same_window {
                self.timer_reset();
                self.last_active_window = info_clone;
            }
            self.record_tracker.ping(info)
        };

        match record {
            Some(ref mut rec) => {
                self.record_classifier.classify(rec);
                self.record_store.push_record(rec.clone())?;

            },
            _ => {}
        };

        Ok({})
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
        // TODO: change this to a function that fetches top record from record store
        self.record_tracker.get_current_archetype()
    }

    fn timer_reset(&mut self) {
        self.last_moment_active = time::SystemTime::now();
    }
}
