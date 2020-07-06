mod config;
mod db;
mod store;
mod tracker;
mod utils;

use std::time::SystemTime;

pub use self::config::*;
pub use self::{
    store::RecordStore,
    tracker::RecordTracker,
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ActivityRecord {
    pub time_range: (SystemTime, SystemTime),
    pub is_productive: Option<bool>,
    pub archetype: Archetype,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Archetype {
    /// Stores title, app name and app class in that order
    ActiveWindow(String, String, String),
    AFK,
}
