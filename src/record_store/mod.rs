mod tracker;
mod store;
mod utils;
mod config;
mod db;

use std::time::{
    SystemTime
};
use chrono::NaiveDate;

pub use self::{
    config::*
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ActivityRecord {
    time_range: (SystemTime, SystemTime),
    archetype: Archetype
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Archetype {
    // Stores title, app name and app class in that order
    ActiveWindow(String, String, String),
    AFK
}
