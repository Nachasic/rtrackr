mod config;
mod db;
mod store;
mod tracker;
mod utils;

use chrono::NaiveDate;
use std::time::SystemTime;

pub use self::config::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ActivityRecord {
    time_range: (SystemTime, SystemTime),
    archetype: Archetype,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Archetype {
    // Stores title, app name and app class in that order
    ActiveWindow(String, String, String),
    AFK,
}
