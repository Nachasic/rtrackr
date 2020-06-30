mod tracker;
mod store;

use std::time::{
    Duration,
    SystemTime
};

pub struct ActivityRecord <'a> {
    time_range: (SystemTime, SystemTime),
    archetype: Archetype<'a>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Archetype <'a> {
    // Stores title, app name and app class in that order
    ActiveWindow(&'a str, &'a str, &'a str),
    AFK
}

