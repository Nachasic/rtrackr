mod tracker;
mod store;
mod utils;
mod crutch;

use std::time::{
    SystemTime
};
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRecord {
    time_range: (SystemTime, SystemTime),
    archetype: Archetype
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Archetype {
    // Stores title, app name and app class in that order
    ActiveWindow(String, String, String),
    AFK
}
