mod config;
mod db;
mod store;
mod tracker;
mod utils;

use std::time::SystemTime;

use crate::classifier::Classifiable;

pub use self::config::*;
pub use self::{
    store::RecordStore,
    tracker::RecordTracker,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductivityStatus {
    Leisure(String),
    Neutral,
    Productive(String)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ActivityRecord {
    pub time_range: (SystemTime, SystemTime),
    pub productivity: ProductivityStatus,
    pub archetype: Archetype,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Archetype {
    /// Stores title, app name and app class in that order
    ActiveWindow(String, String, String),
    AFK,
}

impl Classifiable for ActivityRecord {
    fn get_archetype(&self) -> &Archetype {
        &self.archetype
    }

    fn assign_productivity(&mut self, productivity: ProductivityStatus) {
        self.productivity = productivity;
    }
}
