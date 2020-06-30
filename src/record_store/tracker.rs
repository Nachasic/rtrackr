use std::time::{
    SystemTime
};
use super::*;


#[derive(Debug)]
pub struct RecordTracker <'a> {
    current_archetype: Option<Archetype<'a>>,
    time_of_first_submission: SystemTime,
}

impl <'a> RecordTracker <'a> {
    pub fn new() -> Self {
        Self {
            current_archetype: None,
            time_of_first_submission: SystemTime::now()
        }
    }

    pub fn ping(&'a mut self, arch: Archetype<'a>) -> Option<ActivityRecord> {
        let current = &self.current_archetype;

        match current {
            Some(ref cur_arch) => {
                if cur_arch != &arch {
                    let archetype = cur_arch.clone();
                    let start_time = self.time_of_first_submission.clone();
                    let record = Self::produce_record(archetype, start_time);

                    self.current_archetype = Some(arch);
                    self.time_of_first_submission = SystemTime::now();
                    return Some(record)
                } else {
                    return None
                }
            },
            None => {
                self.current_archetype = Some(arch);
                self.time_of_first_submission = SystemTime::now();
                return None
            }
        }
    }

    fn produce_record(archetype: Archetype, start_time: SystemTime) -> ActivityRecord {
        let end_time = SystemTime::now();

        ActivityRecord {
            archetype,
            time_range: (start_time, end_time)
        }
    }
}

#[test]
fn basic_test() {
    let mut tracker = RecordTracker::new();
    let arch = Archetype::ActiveWindow("title", "my_app", "basic app");

    let report = tracker.ping(arch);

    assert!(match report {
        None => true,
        _ => false
    })
}